//! A client's window, as s&box asks SDL for one: made, shown, sized,
//! focused, its keyboard and mouse handed to managed as SDL3's events, its
//! swap chain a wgpu surface the main camera draws to each frame.
//!
//! Windows here are winit's, on a thread of their own (managed has the main
//! thread; X11, Wayland and Windows allow it, macOS doesn't yet). What the
//! window hears is queued as SDL events, 128 bytes each as SDL3 lays them
//! out, for managed to poll (`SdlEvents.Poll`). Its pixels arrive two ways:
//! the startup splash's (`PresentWindowPixels`), then each frame the main
//! camera's view (`CCameraRenderer::Render` to its swap chain), shown at
//! `Present`.
use std::collections::{HashMap, VecDeque};
use std::ffi::CString;
use std::sync::mpsc;
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{CursorGrabMode, Window, WindowId};

// SDL3's event types (managed's Sdl.EventType)
const QUIT: u32 = 0x100;
const WINDOW_SHOWN: u32 = 0x202;
const WINDOW_EXPOSED: u32 = 0x204;
const WINDOW_RESIZED: u32 = 0x206;
const WINDOW_PIXEL_SIZE_CHANGED: u32 = 0x207;
const WINDOW_MOUSE_ENTER: u32 = 0x20c;
const WINDOW_MOUSE_LEAVE: u32 = 0x20d;
const WINDOW_FOCUS_GAINED: u32 = 0x20e;
const WINDOW_FOCUS_LOST: u32 = 0x20f;
const WINDOW_CLOSE_REQUESTED: u32 = 0x210;
const KEY_DOWN: u32 = 0x300;
const KEY_UP: u32 = 0x301;
const TEXT_INPUT: u32 = 0x303;
const MOUSE_MOTION: u32 = 0x400;
const MOUSE_BUTTON_DOWN: u32 = 0x401;
const MOUSE_BUTTON_UP: u32 = 0x402;
const MOUSE_WHEEL: u32 = 0x403;

// SDL_WindowFlags
const FLAG_HIDDEN: i64 = 0x8;
const FLAG_BORDERLESS: i64 = 0x10;
const FLAG_RESIZABLE: i64 = 0x20;
const FLAG_INPUT_FOCUS: i64 = 0x200;
const FLAG_MOUSE_FOCUS: i64 = 0x400;
const FLAG_FULLSCREEN: i64 = 0x1;
const FLAG_RELATIVE: i64 = 0x8000;
const FLAG_VULKAN: i64 = 0x1000_0000;

/// One SDL event, as SDL3 lays it out: 128 bytes.
#[derive(Clone, Copy)]
pub struct Event(pub [u8; 128]);

impl Event {
    fn new(kind: u32, window: u32) -> Event {
        let mut e = Event([0; 128]);
        e.put_u32(0, kind);
        let ns = START.get_or_init(std::time::Instant::now).elapsed().as_nanos() as u64;
        e.0[8..16].copy_from_slice(&ns.to_le_bytes());
        e.put_u32(16, window);
        e
    }
    fn put_u32(&mut self, at: usize, v: u32) { self.0[at..at + 4].copy_from_slice(&v.to_le_bytes()) }
    fn put_i32(&mut self, at: usize, v: i32) { self.0[at..at + 4].copy_from_slice(&v.to_le_bytes()) }
    fn put_f32(&mut self, at: usize, v: f32) { self.0[at..at + 4].copy_from_slice(&v.to_le_bytes()) }
    fn put_u64(&mut self, at: usize, v: u64) { self.0[at..at + 8].copy_from_slice(&v.to_le_bytes()) }
    pub fn kind(&self) -> u32 { u32::from_le_bytes(self.0[0..4].try_into().unwrap()) }
    pub fn window(&self) -> u32 { u32::from_le_bytes(self.0[16..20].try_into().unwrap()) }
}

static START: OnceLock<std::time::Instant> = OnceLock::new();

/// A window, by the pointer managed holds (an `SDL_Window*` to it).
pub struct Win {
    pub id: u32,
    pub window: Arc<Window>,
    pub state: Mutex<WinState>,
}

#[derive(Default)]
pub struct WinState {
    pub focused: bool,
    pub mouse_inside: bool,
    pub relative: bool,
    pub hidden: bool,
    pub fullscreen: bool,
    pub borderless: bool,
    /// the cursor, in the window's pixels
    pub mouse: (f32, f32),
    pub buttons: u32,
    /// the startup splash's software surface, until a swap chain replaces it
    pub software: bool,
    pub surface: Option<Surface>,
    /// the main camera's last frame, drawn into the surface, waiting for `Present`
    pub frame: Option<wgpu::SurfaceTexture>,
}

pub struct Surface {
    pub surface: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,
}

#[derive(Default)]
struct Windows {
    by_ptr: HashMap<usize, Arc<Win>>,
    by_id: HashMap<WindowId, usize>,
    next: u32,
    /// what managed hasn't polled yet
    events: VecDeque<Event>,
    /// text events' strings, kept until the next poll (SDL lends them so long)
    lent: Vec<CString>,
    error: CString,
}

fn windows() -> MutexGuard<'static, Windows> {
    static W: OnceLock<Mutex<Windows>> = OnceLock::new();
    W.get_or_init(Default::default).lock().unwrap_or_else(|e| e.into_inner())
}

/// Whether this process has made a window (a client that draws).
pub fn any() -> bool {
    windows().next > 0
}

pub fn get(p: usize) -> Option<Arc<Win>> {
    windows().by_ptr.get(&p).cloned()
}

fn by_window_id(id: u32) -> usize {
    windows().by_ptr.iter().find(|(_, w)| w.id == id).map(|(p, _)| *p).unwrap_or(0)
}

fn push(e: Event) {
    windows().events.push_back(e);
}

pub fn set_error(e: &str) {
    windows().error = CString::new(e.replace('\0', "")).unwrap_or_default();
}

pub fn error_ptr() -> usize {
    windows().error.as_ptr() as usize
}

// ---- the event loop's thread

enum Request {
    Create { title: String, width: u32, height: u32, hidden: bool, borderless: bool, reply: mpsc::Sender<Result<Arc<Window>, String>> },
}

struct App;

fn proxy() -> Result<&'static EventLoopProxy<Request>, String> {
    static P: OnceLock<Result<EventLoopProxy<Request>, String>> = OnceLock::new();
    P.get_or_init(|| {
        let (tx, rx) = mpsc::channel();
        std::thread::Builder::new().name("windows".into()).spawn(move || {
            let mut b = EventLoop::<Request>::with_user_event();
            #[cfg(all(unix, not(target_os = "macos")))]
            { use winit::platform::x11::EventLoopBuilderExtX11; b.with_any_thread(true); }
            #[cfg(windows)]
            { use winit::platform::windows::EventLoopBuilderExtWindows; b.with_any_thread(true); }
            match b.build() {
                Ok(el) => {
                    let _ = tx.send(Ok(el.create_proxy()));
                    let _ = el.run_app(&mut App);
                }
                Err(e) => { let _ = tx.send(Err(e.to_string())); }
            }
        }).map_err(|e| e.to_string())?;
        rx.recv().map_err(|e| e.to_string())?
    }).as_ref().map_err(|e| e.clone())
}

impl ApplicationHandler<Request> for App {
    fn resumed(&mut self, _: &ActiveEventLoop) {}

    fn user_event(&mut self, el: &ActiveEventLoop, r: Request) {
        match r {
            Request::Create { title, width, height, hidden, borderless, reply } => {
                let attrs = Window::default_attributes()
                    .with_title(title)
                    .with_inner_size(winit::dpi::PhysicalSize::new(width.max(1), height.max(1)))
                    .with_visible(!hidden)
                    .with_decorations(!borderless);
                let _ = reply.send(el.create_window(attrs).map(Arc::new).map_err(|e| e.to_string()));
            }
        }
    }

    fn window_event(&mut self, _: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let Some(p) = windows().by_id.get(&id).copied() else { return };
        let Some(w) = get(p) else { return };
        let wid = w.id;
        match event {
            WindowEvent::Resized(size) => {
                for kind in [WINDOW_RESIZED, WINDOW_PIXEL_SIZE_CHANGED] {
                    let mut e = Event::new(kind, wid);
                    e.put_i32(20, size.width as i32);
                    e.put_i32(24, size.height as i32);
                    push(e);
                }
            }
            WindowEvent::RedrawRequested => push(Event::new(WINDOW_EXPOSED, wid)),
            WindowEvent::Focused(on) => {
                w.state.lock().unwrap().focused = on;
                push(Event::new(if on { WINDOW_FOCUS_GAINED } else { WINDOW_FOCUS_LOST }, wid));
            }
            WindowEvent::CursorEntered { .. } => { w.state.lock().unwrap().mouse_inside = true; push(Event::new(WINDOW_MOUSE_ENTER, wid)) }
            WindowEvent::CursorLeft { .. } => { w.state.lock().unwrap().mouse_inside = false; push(Event::new(WINDOW_MOUSE_LEAVE, wid)) }
            WindowEvent::CloseRequested => {
                push(Event::new(WINDOW_CLOSE_REQUESTED, wid));
                push(Event::new(QUIT, 0));
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let PhysicalKey::Code(code) = event.physical_key else { return };
                let scancode = scancode(code);
                if scancode == 0 { return; }
                let down = event.state == ElementState::Pressed;
                let mut e = Event::new(if down { KEY_DOWN } else { KEY_UP }, wid);
                e.put_u32(20, 0);
                e.put_i32(24, scancode as i32);
                e.put_u32(28, keycode(scancode));
                e.0[32..34].copy_from_slice(&modifiers().to_le_bytes());
                e.0[36] = down as u8;
                e.0[37] = event.repeat as u8;
                push(e);
                if down && !event.repeat {
                    if let Some(text) = event.text.filter(|t| !t.chars().any(|c| c.is_control())) {
                        let mut t = Event::new(TEXT_INPUT, wid);
                        let s = CString::new(text.as_str()).unwrap_or_default();
                        let mut all = windows();
                        t.put_u64(24, s.as_ptr() as u64);
                        all.lent.push(s);
                        all.events.push_back(t);
                    }
                }
            }
            WindowEvent::ModifiersChanged(m) => {
                let s = m.state();
                let mut bits = 0u16;
                if s.shift_key() { bits |= 0x0001; }
                if s.control_key() { bits |= 0x0040; }
                if s.alt_key() { bits |= 0x0100; }
                if s.super_key() { bits |= 0x0400; }
                MODS.store(bits as u32, std::sync::atomic::Ordering::Relaxed);
            }
            WindowEvent::CursorMoved { position, .. } => {
                let (x, y) = (position.x as f32, position.y as f32);
                let mut st = w.state.lock().unwrap();
                let (px, py) = st.mouse;
                st.mouse = (x, y);
                // in relative mode, the device's own motion is what's sent
                if st.relative { return; }
                let buttons = st.buttons;
                drop(st);
                let mut e = Event::new(MOUSE_MOTION, wid);
                e.put_u32(24, buttons);
                e.put_f32(28, x);
                e.put_f32(32, y);
                e.put_f32(36, x - px);
                e.put_f32(40, y - py);
                push(e);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let b: u8 = match button { MouseButton::Left => 1, MouseButton::Middle => 2, MouseButton::Right => 3, MouseButton::Back => 4, MouseButton::Forward => 5, MouseButton::Other(_) => return };
                let down = state == ElementState::Pressed;
                let mut st = w.state.lock().unwrap();
                if down { st.buttons |= 1 << (b - 1) } else { st.buttons &= !(1 << (b - 1)) }
                let (x, y) = st.mouse;
                drop(st);
                let mut e = Event::new(if down { MOUSE_BUTTON_DOWN } else { MOUSE_BUTTON_UP }, wid);
                e.0[24] = b;
                e.0[25] = down as u8;
                e.0[26] = 1;
                e.put_f32(28, x);
                e.put_f32(32, y);
                push(e);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let (x, y) = match delta { MouseScrollDelta::LineDelta(x, y) => (x, y), MouseScrollDelta::PixelDelta(p) => (p.x as f32 / 32.0, p.y as f32 / 32.0) };
                let (mx, my) = w.state.lock().unwrap().mouse;
                let mut e = Event::new(MOUSE_WHEEL, wid);
                e.put_f32(24, x);
                e.put_f32(28, y);
                e.put_u32(32, 0);
                e.put_f32(36, mx);
                e.put_f32(40, my);
                e.put_i32(44, x as i32);
                e.put_i32(48, y as i32);
                push(e);
            }
            _ => {}
        }
    }

    /// The mouse's own motion: what relative mode (mouse look) is made of.
    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        let DeviceEvent::MouseMotion { delta } = event else { return };
        let target = windows().by_ptr.values().find(|w| w.state.lock().unwrap().relative).cloned();
        let Some(w) = target else { return };
        let buttons = w.state.lock().unwrap().buttons;
        let mut e = Event::new(MOUSE_MOTION, w.id);
        e.put_u32(24, buttons);
        e.put_f32(36, delta.0 as f32);
        e.put_f32(40, delta.1 as f32);
        push(e);
    }
}

static MODS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

pub fn modifiers() -> u16 {
    MODS.load(std::sync::atomic::Ordering::Relaxed) as u16
}

// ---- what managed asks

/// A window (CreateWindow, CreateStartupWindow): its pointer, or 0 and why.
pub fn create(title: &str, width: i32, height: i32, flags: i64, software: bool) -> usize {
    let proxy = match proxy() {
        Ok(p) => p,
        Err(e) => { set_error(&format!("no windows here: {e}")); return 0; }
    };
    let (reply, got) = mpsc::channel();
    let hidden = flags & FLAG_HIDDEN != 0;
    let borderless = flags & FLAG_BORDERLESS != 0;
    if proxy.send_event(Request::Create { title: title.to_string(), width: width.max(1) as u32, height: height.max(1) as u32, hidden, borderless, reply }).is_err() {
        set_error("the window thread has gone");
        return 0;
    }
    let window = match got.recv() {
        Ok(Ok(w)) => w,
        Ok(Err(e)) => { set_error(&e); return 0; }
        Err(_) => { set_error("the window thread didn't answer"); return 0; }
    };
    let p = crate::handles::object();
    let mut all = windows();
    all.next += 1;
    let id = all.next;
    all.by_id.insert(window.id(), p);
    let state = WinState { hidden, borderless, software, ..Default::default() };
    all.by_ptr.insert(p, Arc::new(Win { id, window, state: Mutex::new(state) }));
    crate::trace::note(&format!("window: {title:?}, {width}x{height}"));
    p
}

pub fn destroy(p: usize) {
    let mut all = windows();
    if let Some(w) = all.by_ptr.remove(&p) { all.by_id.remove(&w.window.id()); }
}

/// The next event for managed (PollEvent): written to `out` (128 bytes).
pub fn poll(out: usize) -> bool {
    let mut all = windows();
    // a text event's string is lent until the next poll: the last few kept
    let over = all.lent.len().saturating_sub(64);
    all.lent.drain(..over);
    let Some(e) = all.events.pop_front() else { return false };
    if out != 0 { unsafe { core::ptr::copy_nonoverlapping(e.0.as_ptr(), out as *mut u8, 128) } }
    true
}

pub fn window_of_event(event: usize) -> usize {
    if event == 0 { return 0; }
    let id = unsafe { *((event + 16) as *const u32) };
    if id == 0 { 0 } else { by_window_id(id) }
}

pub fn size(p: usize) -> (i32, i32) {
    get(p).map(|w| { let s = w.window.inner_size(); (s.width as i32, s.height as i32) }).unwrap_or((0, 0))
}

pub fn flags(p: usize) -> i64 {
    let Some(w) = get(p) else { return 0 };
    let s = w.state.lock().unwrap();
    let mut f = FLAG_VULKAN | FLAG_RESIZABLE;
    if s.hidden { f |= FLAG_HIDDEN; }
    if s.borderless { f |= FLAG_BORDERLESS; }
    if s.fullscreen { f |= FLAG_FULLSCREEN; }
    if s.focused { f |= FLAG_INPUT_FOCUS; }
    if s.mouse_inside || s.relative { f |= FLAG_MOUSE_FOCUS; }
    if s.relative { f |= FLAG_RELATIVE; }
    f
}

/// The window with the keyboard's focus (0: none of ours).
pub fn keyboard_focus() -> usize {
    windows().by_ptr.iter().find(|(_, w)| w.state.lock().unwrap().focused).map(|(p, _)| *p).unwrap_or(0)
}

pub fn mouse_focus() -> usize {
    windows().by_ptr.iter().find(|(_, w)| { let s = w.state.lock().unwrap(); s.mouse_inside || s.relative }).map(|(p, _)| *p).unwrap_or(0)
}

pub fn with<T>(p: usize, f: impl FnOnce(&Win, &mut WinState) -> T) -> Option<T> {
    let w = get(p)?;
    let mut s = w.state.lock().unwrap();
    Some(f(&w, &mut s))
}

pub fn show(p: usize, visible: bool) -> bool {
    with(p, |w, s| {
        s.hidden = !visible;
        w.window.set_visible(visible);
        if visible {
            w.window.focus_window();
            let id = w.id;
            push(Event::new(WINDOW_SHOWN, id));
            push(Event::new(WINDOW_EXPOSED, id));
        }
    }).is_some()
}

/// Mouse look: the cursor held and hidden, the device's motion sent.
pub fn set_relative(p: usize, on: bool) -> bool {
    with(p, |w, s| {
        if on {
            let held = w.window.set_cursor_grab(CursorGrabMode::Locked).or_else(|_| w.window.set_cursor_grab(CursorGrabMode::Confined));
            if held.is_err() { crate::trace::note("window: the cursor can't be held here; relative motion still sent"); }
        } else {
            let _ = w.window.set_cursor_grab(CursorGrabMode::None);
        }
        w.window.set_cursor_visible(!on);
        s.relative = on;
        true
    }).unwrap_or(false)
}

pub fn relative(p: usize) -> bool {
    with(p, |_, s| s.relative).unwrap_or(false)
}

// ---- pixels: the splash's, and the main camera's at Present

/// A window's surface, made (or remade at a new size) on the renderer's device.
fn surface_for(w: &Win, s: &mut WinState, r: &crate::render::Renderer, width: u32, height: u32) -> Option<()> {
    if s.surface.as_ref().is_some_and(|x| x.config.width == width && x.config.height == height) { return Some(()); }
    if s.surface.is_none() {
        let surface = r.instance.create_surface(w.window.clone()).map_err(|e| crate::trace::note(&format!("window: no surface: {e}"))).ok()?;
        let caps = surface.get_capabilities(&r.gpu);
        let format = caps.formats.iter().copied().find(|f| matches!(f, wgpu::TextureFormat::Bgra8Unorm | wgpu::TextureFormat::Rgba8Unorm)).or(caps.formats.first().copied())?;
        if !caps.usages.contains(wgpu::TextureUsages::COPY_DST) {
            crate::trace::note("window: this surface can't be copied to");
            return None;
        }
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: caps.alpha_modes[0],
            // drawn through an sRGB view, as the renderer's own output is
            view_formats: vec![format.add_srgb_suffix()],
        };
        crate::trace::note(&format!("window: a surface, {format:?}, {width}x{height}"));
        s.surface = Some(Surface { surface, config });
    }
    let x = s.surface.as_mut()?;
    x.config.width = width;
    x.config.height = height;
    x.surface.configure(&r.device, &x.config);
    Some(())
}

/// RGBA pixels (top row first) shown in a window now.
pub fn show_pixels(p: usize, rgba: &[u8], width: u32, height: u32) -> bool {
    let Some(r) = crate::camera::shared_renderer() else { return false };
    with(p, |w, s| {
        if width == 0 || height == 0 || rgba.len() < (width * height * 4) as usize { return false; }
        if surface_for(w, s, r, width, height).is_none() { return false; }
        let x = s.surface.as_ref().unwrap();
        let frame = match x.surface.get_current_texture() {
            Ok(f) => f,
            Err(e) => {
                x.surface.configure(&r.device, &x.config);
                crate::trace::note(&format!("window: no frame to draw to: {e}"));
                return false;
            }
        };
        let bgra = matches!(x.config.format, wgpu::TextureFormat::Bgra8Unorm | wgpu::TextureFormat::Bgra8UnormSrgb);
        let pixels: std::borrow::Cow<[u8]> = if bgra { rgba.chunks_exact(4).flat_map(|c| [c[2], c[1], c[0], c[3]]).collect::<Vec<u8>>().into() } else { rgba.into() };
        r.queue.write_texture(
            wgpu::TexelCopyTextureInfo { texture: &frame.texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            &pixels,
            wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(width * 4), rows_per_image: Some(height) },
            wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        );
        r.queue.submit([]);
        w.window.pre_present_notify();
        frame.present();
        true
    }).unwrap_or(false)
}

// ---- swap chains: a window to draw to, and what the camera drew

/// SwapChainHandle_t: the window it shows, by a handle of its own.
pub fn swap_chains() -> MutexGuard<'static, HashMap<usize, usize>> {
    static S: OnceLock<Mutex<HashMap<usize, usize>>> = OnceLock::new();
    S.get_or_init(Default::default).lock().unwrap_or_else(|e| e.into_inner())
}

pub fn create_swap_chain(window: usize) -> usize {
    if get(window).is_none() { return 0; }
    with(window, |_, s| s.software = false);
    let h = crate::handles::object();
    swap_chains().insert(h, window);
    h
}

pub fn window_of_swap_chain(sc: usize) -> usize {
    swap_chains().get(&sc).copied().unwrap_or(0)
}

/// The main camera's view for a swap chain (SceneCamera.Render): drawn
/// straight into the window's next frame, shown at Present.
pub fn draw_camera(camera: usize, sc: usize) {
    let window = window_of_swap_chain(sc);
    let (width, height) = size(window);
    if width <= 1 || height <= 1 { return; }
    let Some(r) = crate::camera::shared_renderer() else { return };
    let began = std::time::Instant::now();
    let frame = with(window, |w, s| {
        surface_for(w, s, r, width as u32, height as u32)?;
        let x = s.surface.as_ref()?;
        match x.surface.get_current_texture() {
            Ok(f) => Some((f, x.config.format.add_srgb_suffix())),
            Err(e) => {
                x.surface.configure(&r.device, &x.config);
                crate::trace::note(&format!("window: no frame to draw to: {e}"));
                None
            }
        }
    }).flatten();
    let Some((frame, format)) = frame else { return };
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor { format: Some(format), ..Default::default() });
    let Some((_, said)) = crate::camera::frame_into(camera, width, height, Some((&view, format))) else { return };
    static FRAMES: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let n = FRAMES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if n % 30 == 0 { crate::trace::note(&format!("window frame {n} ({} ms): {said}", began.elapsed().as_millis())); }
    // the UI, while the device it needs is built out (ENGINE2_GRAPHICS=1): recorded, not yet drawn
    let graphics = std::env::var("ENGINE2_GRAPHICS").is_ok_and(|v| v == "1");
    let ui = if graphics { ui_draws(camera, width, height) } else { Vec::new() };
    if !ui.is_empty() {
        let batched = to_draw(&ui);
        let linear = frame.texture.create_view(&wgpu::TextureViewDescriptor { format: Some(format.remove_srgb_suffix()), ..Default::default() });
        r.ui.draw(&r.device, &r.queue, &linear, format.remove_srgb_suffix(), width as u32, height as u32, &batched);
    }
    static SAID: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    if !ui.is_empty() && (SAID.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 3 || n % 60 == 0) {
        let mut by: std::collections::BTreeMap<String, (usize, i32)> = Default::default();
        for d in &ui {
            let e = by.entry(crate::camera::material_shader(d.material).unwrap_or_else(|| format!("{:#x}", d.material))).or_default();
            e.0 += 1;
            e.1 += d.instances;
        }
        crate::trace::note(&format!("window frame {n}: the UI drew {by:?}"));
        if std::env::var("ENGINE2_UI_DEBUG").is_ok() { ui_dump(&ui[0]); }
    }
    with(window, |_, s| s.frame = Some(frame));
}

/// The batched UI box's draws, their tables found by their attributes'
/// names (the shader's): what render/ui.rs draws.
fn to_draw(draws: &[crate::gpu::Draw]) -> Vec<crate::render::ui::UiDraw> {
    use crate::gpu::Val;
    // each table as it was when the draw was made
    let bytes = |d: &crate::gpu::Draw, name: &str| match d.attrs.get(&crate::string_token(name)) {
        Some(Val::Buffer(b)) => d.buffers.get(b).map(|x| x.as_ref().clone()).unwrap_or_default(),
        _ => Vec::new(),
    };
    draws.iter().filter(|d| crate::camera::material_shader(d.material).is_some_and(|s| s.contains("ui_cssbox_batched"))).map(|d| {
        let layer = match d.attrs.get(&crate::string_token("LayerMat")) { Some(Val::Matrix(m)) => *m, _ => [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0] };
        let instance_offset = match d.attrs.get(&crate::string_token("InstanceOffset")) { Some(Val::Int(o)) => *o, _ => 0 };
        crate::render::ui::UiDraw {
            boxes: bytes(d, "BoxInstances"),
            texts: bytes(d, "PainterTextInstances"),
            glyph_table: bytes(d, "GlyphTable"),
            glyph_curves: bytes(d, "GlyphCurves"),
            glyph_bands: bytes(d, "GlyphBands"),
            transforms: bytes(d, "TransformBuffer"),
            layer,
            instance_offset,
            instances: d.instances.max(0) as u32,
        }
    }).collect()
}

/// What a UI draw holds (ENGINE2_UI_DEBUG): its attributes by the shader's
/// names, and its boxes.
fn ui_dump(d: &crate::gpu::Draw) {
    let names = "BorderShapeBuffer BoxInstances BoxPosition BoxSize GlyphBands GlyphCurves GlyphTable GradientBuffer HasScissor InstanceOffset LayerMat PainterScreenViewport PainterTextInstances PathBuffer PathNodeBuffer PolygonPointBuffer ScissorBuffer ScissorCount TextHeight TextInstanceOffset TextInstances TextTileOffset TextTiles TextTilesX TextWidth TransformBuffer TransformMat UIFrameGrabEncoded UIGammaOutput UIInPanelLayer UIPanelOpacity WorldMat WorldPanelLighting D_BLENDMODE D_JOIN D_NO_ZTEST D_PAINTER_SCREENPIXELS D_PANEL_OPACITY D_POLYGON_POINTS D_WORLDPANEL";
    for n in names.split(' ') {
        if let Some(v) = d.attrs.get(&crate::string_token(n)) {
            let extra = match v { crate::gpu::Val::Buffer(b) => d.buffers.get(b).map(|x| format!(" ({} bytes)", x.len())).unwrap_or_default(), _ => String::new() };
            eprintln!("{}", &format!("ui attr {n} = {v:?}{extra}"));
        }
    }
    if let Some(crate::gpu::Val::Buffer(t)) = d.attrs.get(&crate::string_token("TransformBuffer")) {
        if let Some(data) = d.buffers.get(t) {
            let m: &[f32] = bytemuck::cast_slice(&data[..data.len() / 4 * 4]);
            for k in 0..4 { if m.len() >= k * 16 + 16 { eprintln!("{}", &format!("ui transform {k}: translate {:?} diag {:?}", &m[k * 16 + 12..k * 16 + 15], [m[k * 16], m[k * 16 + 5]])); } }
        }
    }
    let Some(crate::gpu::Val::Buffer(b)) = d.attrs.get(&crate::string_token("BoxInstances")) else { return };
    let Some(data) = d.buffers.get(b) else { return };
    struct B<'a> { data: &'a [u8], element_size: usize, elements: usize }
    let buf = B { data, element_size: 272, elements: data.len() / 272 };
    let f = |i: usize, at: usize| f32::from_le_bytes(buf.data[i * buf.element_size + at..][..4].try_into().unwrap());
    let u = |i: usize, at: usize| u32::from_le_bytes(buf.data[i * buf.element_size + at..][..4].try_into().unwrap());
    let offset = match d.attrs.get(&crate::string_token("InstanceOffset")) { Some(crate::gpu::Val::Int(o)) => *o as usize, _ => 0 };
    for i in offset..(offset + d.instances as usize).min(buf.elements).min(offset + 40) {
        eprintln!("{}", &format!("ui box {i}: rect {:.0} {:.0} {:.0} {:.0} color {:.2} {:.2} {:.2} {:.2} radius {:.0} border {:.0} tex {} flags {:#x} transform {} shape {}",
            f(i, 0), f(i, 4), f(i, 8), f(i, 12), f(i, 16), f(i, 20), f(i, 24), f(i, 28), f(i, 32), f(i, 64), u(i, 144) as i32, u(i, 228), u(i, 236) as i32, u(i, 268) as i32));
    }
}

/// The camera's UI, as the engine's pipeline asks for it: a view of the
/// camera, managed's UI hook added to it (RenderPipeline.AddUIHook), each
/// layer it made run with a render context that records what it draws.
fn ui_draws(camera: usize, width: i32, height: i32) -> Vec<crate::gpu::Draw> {
    use crate::gpu;
    let add = crate::managed(crate::slots::M_SndbxRndrng_RenderPipeline_AddUIHook);
    if add == 0 { return Vec::new(); }
    #[repr(C)]
    struct Viewport { version: i32, x: i32, y: i32, w: i32, h: i32, min_z: f32, max_z: f32 }
    #[repr(C)]
    struct Setup { context: usize, view: usize, layer: usize, format: i32, msaa: i32, stats: usize }
    let view = gpu::new_view(crate::impls::camera_id(camera), width, height);
    let add: extern "C" fn(*const core::ffi::c_char, i64, usize, Viewport, usize) -> usize = unsafe { std::mem::transmute(add) };
    const UI: i64 = 7500;
    add(c"UI".as_ptr(), UI, view, Viewport { version: 0, x: 0, y: 0, w: width, h: height, min_z: 0.0, max_z: 1.0 }, 0);
    let layers: Vec<(usize, usize)> = {
        let d = gpu::device();
        d.views.get(&view).map(|v| v.layers.iter().filter_map(|l| d.layers.get(l).map(|x| (*l, x.callback))).collect()).unwrap_or_default()
    };
    let mut draws = Vec::new();
    for (layer, callback) in layers {
        if callback == 0 { continue; }
        let context = gpu::new_context();
        let run: extern "C" fn(Setup) = unsafe { std::mem::transmute(callback) };
        // ImageFormat RGBA8888 (0), no multisampling (0)
        run(Setup { context, view, layer, format: 0, msaa: 0, stats: 0 });
        if let Some(c) = gpu::take_context(context) { draws.extend(c.draws); }
    }
    gpu::free_view(view);
    draws
}

pub fn present(sc: usize) -> bool {
    let window = window_of_swap_chain(sc);
    let Some(frame) = with(window, |_, s| s.frame.take()).flatten() else { return true };
    if let Some(w) = get(window) { w.window.pre_present_notify(); }
    frame.present();
    true
}

// ---- keys: winit's physical keys as SDL's scancodes (USB HID usages), and
// the keycodes of a US layout

pub fn scancode(k: KeyCode) -> u32 {
    use KeyCode::*;
    match k {
        KeyA => 4, KeyB => 5, KeyC => 6, KeyD => 7, KeyE => 8, KeyF => 9, KeyG => 10, KeyH => 11, KeyI => 12, KeyJ => 13,
        KeyK => 14, KeyL => 15, KeyM => 16, KeyN => 17, KeyO => 18, KeyP => 19, KeyQ => 20, KeyR => 21, KeyS => 22, KeyT => 23,
        KeyU => 24, KeyV => 25, KeyW => 26, KeyX => 27, KeyY => 28, KeyZ => 29,
        Digit1 => 30, Digit2 => 31, Digit3 => 32, Digit4 => 33, Digit5 => 34, Digit6 => 35, Digit7 => 36, Digit8 => 37, Digit9 => 38, Digit0 => 39,
        Enter => 40, Escape => 41, Backspace => 42, Tab => 43, Space => 44, Minus => 45, Equal => 46, BracketLeft => 47, BracketRight => 48,
        Backslash => 49, Semicolon => 51, Quote => 52, Backquote => 53, Comma => 54, Period => 55, Slash => 56, CapsLock => 57,
        F1 => 58, F2 => 59, F3 => 60, F4 => 61, F5 => 62, F6 => 63, F7 => 64, F8 => 65, F9 => 66, F10 => 67, F11 => 68, F12 => 69,
        PrintScreen => 70, ScrollLock => 71, Pause => 72, Insert => 73, Home => 74, PageUp => 75, Delete => 76, End => 77, PageDown => 78,
        ArrowRight => 79, ArrowLeft => 80, ArrowDown => 81, ArrowUp => 82,
        NumLock => 83, NumpadDivide => 84, NumpadMultiply => 85, NumpadSubtract => 86, NumpadAdd => 87, NumpadEnter => 88,
        Numpad1 => 89, Numpad2 => 90, Numpad3 => 91, Numpad4 => 92, Numpad5 => 93, Numpad6 => 94, Numpad7 => 95, Numpad8 => 96, Numpad9 => 97, Numpad0 => 98, NumpadDecimal => 99,
        ControlLeft => 224, ShiftLeft => 225, AltLeft => 226, SuperLeft => 227, ControlRight => 228, ShiftRight => 229, AltRight => 230, SuperRight => 231,
        _ => 0,
    }
}

/// SDL's keycode for a scancode on a US layout: printable keys are their
/// character, the rest the scancode with bit 30 set (SDLK_SCANCODE_MASK).
pub fn keycode(scancode: u32) -> u32 {
    match scancode {
        4..=29 => b'a' as u32 + (scancode - 4),
        30..=38 => b'1' as u32 + (scancode - 30),
        39 => b'0' as u32,
        40 => 0x0d, 41 => 0x1b, 42 => 0x08, 43 => 0x09, 44 => b' ' as u32,
        45 => b'-' as u32, 46 => b'=' as u32, 47 => b'[' as u32, 48 => b']' as u32, 49 => b'\\' as u32,
        51 => b';' as u32, 52 => b'\'' as u32, 53 => b'`' as u32, 54 => b',' as u32, 55 => b'.' as u32, 56 => b'/' as u32,
        76 => 0x7f,
        s => s | (1 << 30),
    }
}

/// A key's name, as SDL names them.
pub fn key_name(key: u32) -> String {
    match key {
        0x0d => "Return".into(), 0x1b => "Escape".into(), 0x08 => "Backspace".into(), 0x09 => "Tab".into(), 0x20 => "Space".into(), 0x7f => "Delete".into(),
        k if k < 0x80 => (k as u8 as char).to_ascii_uppercase().to_string(),
        k => match k & !(1 << 30) {
            s @ 58..=69 => format!("F{}", s - 57),
            79 => "Right".into(), 80 => "Left".into(), 81 => "Down".into(), 82 => "Up".into(),
            224 => "Left Ctrl".into(), 225 => "Left Shift".into(), 226 => "Left Alt".into(), 228 => "Right Ctrl".into(), 229 => "Right Shift".into(), 230 => "Right Alt".into(),
            s => format!("Key {s}"),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn events_are_laid_out_as_sdl3s() {
        let mut e = Event::new(KEY_DOWN, 3);
        e.put_i32(24, scancode(KeyCode::KeyW) as i32);
        e.put_u32(28, keycode(26));
        assert_eq!((e.kind(), e.window()), (KEY_DOWN, 3));
        // managed's KeyboardEvent: Common (16), WindowID, Which, Scancode, Key
        assert_eq!(i32::from_le_bytes(e.0[24..28].try_into().unwrap()), 26);
        assert_eq!(u32::from_le_bytes(e.0[28..32].try_into().unwrap()), b'w' as u32);
        assert_eq!(keycode(41), 0x1b);
        assert_eq!(keycode(225), 225 | 1 << 30);
        assert_eq!(key_name(b'w' as u32), "W");
        assert_eq!(key_name(keycode(225)), "Left Shift");
    }

    #[test]
    fn polled_events_come_out_in_order_and_whole() {
        push(Event::new(WINDOW_FOCUS_GAINED, 7));
        let mut k = Event::new(KEY_DOWN, 7);
        k.put_i32(24, 26);
        push(k);
        let mut out = [0u8; 128];
        assert!(poll(out.as_mut_ptr() as usize));
        assert_eq!(u32::from_le_bytes(out[0..4].try_into().unwrap()), WINDOW_FOCUS_GAINED);
        assert!(poll(out.as_mut_ptr() as usize));
        assert_eq!(i32::from_le_bytes(out[24..28].try_into().unwrap()), 26);
        assert!(!poll(out.as_mut_ptr() as usize));
    }
}

// ---- small things managed asks of a window

pub fn set_title(p: usize, title: &str) -> bool { with(p, |w, _| w.window.set_title(title)).is_some() }

pub fn set_size(p: usize, width: i32, height: i32) -> bool {
    with(p, |w, _| { let _ = w.window.request_inner_size(winit::dpi::PhysicalSize::new(width.max(1) as u32, height.max(1) as u32)); }).is_some()
}

pub fn position(p: usize) -> (i32, i32) {
    with(p, |w, _| w.window.outer_position().map(|q| (q.x, q.y)).unwrap_or((0, 0))).unwrap_or((0, 0))
}

pub fn set_position(p: usize, x: i32, y: i32) -> bool { with(p, |w, _| w.window.set_outer_position(winit::dpi::PhysicalPosition::new(x, y))).is_some() }

pub fn set_fullscreen(p: usize, on: bool) -> bool {
    with(p, |w, s| { s.fullscreen = on; w.window.set_fullscreen(on.then(|| winit::window::Fullscreen::Borderless(None))); }).is_some()
}

pub fn set_bordered(p: usize, on: bool) -> bool { with(p, |w, s| { s.borderless = !on; w.window.set_decorations(on) }).is_some() }

pub fn set_resizable(p: usize, on: bool) -> bool { with(p, |w, _| w.window.set_resizable(on)).is_some() }

pub fn raise(p: usize) -> bool { with(p, |w, _| w.window.focus_window()).is_some() }

pub fn warp_mouse(p: usize, x: f32, y: f32) {
    with(p, |w, s| { s.mouse = (x, y); let _ = w.window.set_cursor_position(winit::dpi::PhysicalPosition::new(x as f64, y as f64)); });
}

/// The focused window's cursor and buttons (GetMouseState).
pub fn mouse_state() -> (u32, f32, f32) {
    let p = mouse_focus().max(keyboard_focus());
    with(p, |_, s| (s.buttons, s.mouse.0, s.mouse.1)).unwrap_or((0, 0.0, 0.0))
}

pub fn has_software_surface(p: usize) -> bool { with(p, |_, s| s.software).unwrap_or(false) }

pub fn release_software_surface(p: usize) -> bool { with(p, |_, s| s.software = false).is_some() }

pub fn set_text_input(p: usize, on: bool) -> bool { with(p, |w, _| w.window.set_ime_allowed(on)).is_some() }

pub fn set_cursor_visible(on: bool) {
    for w in windows().by_ptr.values() { w.window.set_cursor_visible(on); }
}

/// The monitor the window's on: its size (or a 1920x1080 guess), and its refresh in mHz.
pub fn monitor() -> (i32, i32, u32) {
    let w = windows().by_ptr.values().next().cloned();
    w.and_then(|w| w.window.current_monitor())
        .map(|m| (m.size().width as i32, m.size().height as i32, m.refresh_rate_millihertz().unwrap_or(60_000)))
        .unwrap_or((1920, 1080, 60_000))
}
