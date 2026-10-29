// Written by gen/ from s&box's managed interop (Interop.Engine.cs): do not edit.
// igen_engine: 3578 native slots, 71 managed callbacks, hash 55185.
#![allow(non_camel_case_types, dead_code, unused_unsafe, clippy::all)]
use crate::trace::hit;
use crate::trace::debug_error;
pub const HASH: i32 = 55185;
pub const MANAGED: usize = 71;
pub const SLOTS: usize = 3578;

pub const M_Sandbox_AsyncGPUReadback_DispatchManagedReadTextureCallback: usize = 0;
pub const M_Sandbox_AsyncGPUReadback_DispatchManagedReadBufferCallback: usize = 1;
pub const M_SandboxAudio_MixingThread_MixOneBuffer: usize = 2;
pub const M_Sandbox_ConVarSystem_RegisterNativeVar: usize = 3;
pub const M_Sandbox_ConVarSystem_RegisterNativeCommand: usize = 4;
pub const M_Sandbox_ConVarSystem_OnConVarChanged: usize = 5;
pub const M_SndbxDgnstcs_Logging_RegisterEngineLogger: usize = 6;
pub const M_SndbxDgnstcs_Logging_PrintNative: usize = 7;
pub const M_SandboxEngine_Bootstrap_EnvironmentExit: usize = 8;
pub const M_SandboxEngine_ConsoleConfig_Initialize: usize = 9;
pub const M_SandboxEngine_GameWindow_Present: usize = 10;
pub const M_SandboxEngine_InputRouter_CloseApplication: usize = 11;
pub const M_SandboxEngine_SystemInfo_Set: usize = 12;
pub const M_SandboxEngine_SystemInfo_SetGpu: usize = 13;
pub const M_SandboxEngine_WindowInput_OnWindowDestroyed: usize = 14;
pub const M_Sandbox_EngineLoop_OnClientOutput: usize = 15;
pub const M_Sandbox_HandleIndex_RegisterHandle: usize = 16;
pub const M_Sandbox_HandleIndex_FreeHandle: usize = 17;
pub const M_Sandbox_LobbyManager_OnLobbyInvite: usize = 18;
pub const M_Sandbox_LobbyManager_OnMemberEntered: usize = 19;
pub const M_Sandbox_LobbyManager_OnMemberLeave: usize = 20;
pub const M_Sandbox_LobbyManager_OnEntered: usize = 21;
pub const M_Sandbox_LobbyManager_OnCreated: usize = 22;
pub const M_Sandbox_LobbyManager_OnDataUpdate: usize = 23;
pub const M_Sandbox_LobbyManager_OnChatMessage: usize = 24;
pub const M_SandboxNetwork_SteamNetwork_OnSocketConnection: usize = 25;
pub const M_SandboxNetwork_SteamNetwork_OnSocketDisconnection: usize = 26;
pub const M_SandboxNetwork_SteamNetwork_ShouldAcceptConnection: usize = 27;
pub const M_SandboxNetwork_SteamNetwork_OnDisconnection: usize = 28;
pub const M_SandboxNetwork_SteamNetwork_OnSessionEstablished: usize = 29;
pub const M_SandboxNetwork_SteamNetwork_OnSessionFailed: usize = 30;
pub const M_SandboxPhysics_PhysicsEngine_OnPhysicsJointBreak: usize = 31;
pub const M_SandboxPhysics_PhysicsEngine_OnActive: usize = 32;
pub const M_Sandbox_RealTime_Update: usize = 33;
pub const M_SndbxRndrng_RenderPipeline_AddLayersToView: usize = 34;
pub const M_SndbxRndrng_RenderPipeline_AddSkybox3DLayersToView: usize = 35;
pub const M_SndbxRndrng_RenderPipeline_OnSceneViewSubmitted: usize = 36;
pub const M_SndbxRndrng_RenderPipeline_AddUIHook: usize = 37;
pub const M_SndbxRndrng_ShdwMpprCllbcks_InitForView: usize = 38;
pub const M_SndbxRndrng_ShdwMpprCllbcks_SetShaderAttributes: usize = 39;
pub const M_SndbxRndrng_ShdwMpprCllbcks_UploadToGPU: usize = 40;
pub const M_SndbxRndrng_ShdwMpprCllbcks_DoDirectionalLight: usize = 41;
pub const M_SndbxRndrng_ShdwMpprCllbcks_FindOrCreateShadowMaps: usize = 42;
pub const M_Sandbox_Resource_OnResourceReloaded: usize = 43;
pub const M_Sandbox_Resource_OnResourceLoaded: usize = 44;
pub const M_Sandbox_Resource_OnGuidChanged: usize = 45;
pub const M_Sandbox_Resource_OnResourcePathChanged: usize = 46;
pub const M_Sandbox_ScnCstmbjctRndr_RenderObject: usize = 47;
pub const M_Sandbox_SceneSystem_OnBeforeRender: usize = 48;
pub const M_Sandbox_SceneSystem_OnAfterRender: usize = 49;
pub const M_SndbxSrvcs_Inventory_OnDefinitionUpdate: usize = 50;
pub const M_SndbxSrvcs_Inventory_OnPricesUpdate: usize = 51;
pub const M_SndbxSrvcs_Inventory_OnPurchaseResult: usize = 52;
pub const M_SndbxSrvcs_ServerList_OnStarted: usize = 53;
pub const M_SndbxSrvcs_ServerList_OnServerResponded: usize = 54;
pub const M_SndbxSrvcs_ServerList_OnFinished: usize = 55;
pub const M_SandboxVR_VRSystem_InternalIsActive: usize = 56;
pub const M_SandboxVR_VRSystem_BeginFrame: usize = 57;
pub const M_SandboxVR_VRSystem_EndFrame: usize = 58;
pub const M_SandboxVR_VRSystem_Submit: usize = 59;
pub const M_SandboxVR_VRSystem_GetVulkanInstanceExtensionsRequired: usize = 60;
pub const M_SandboxVR_VRSystem_GetVulkanDeviceExtensionsRequired: usize = 61;
pub const M_Sandbox_VideoPlayer_OnTextureCreatedInternal: usize = 62;
pub const M_Sandbox_VideoPlayer_OnInitAudioInternal: usize = 63;
pub const M_Sandbox_VideoPlayer_OnFreeAudioInternal: usize = 64;
pub const M_Sandbox_VideoPlayer_OnTextureDataInternal: usize = 65;
pub const M_Sandbox_VideoPlayer_OnLoadedInternal: usize = 66;
pub const M_Sandbox_VideoPlayer_WantsTextureData: usize = 67;
pub const M_Sandbox_VideoPlayer_OnFinishedInternal: usize = 68;
pub const M_Sandbox_VideoPlayer_OnRepeatedInternal: usize = 69;
pub const M_Steamworks_Dispatch_OnClientCallback: usize = 70;

/// System.Numerics.Vector3, 12 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_System_Numerics_Vector3 {
    pub f_X: f32,
    pub f_Y: f32,
    pub f_Z: f32,
}
const _: () = assert!(core::mem::size_of::<S_System_Numerics_Vector3>() == 12);

/// Vector3, 12 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_Vector3 {
    pub f__vec: S_System_Numerics_Vector3,
}
const _: () = assert!(core::mem::size_of::<S_Vector3>() == 12);

/// System.Numerics.Quaternion, 16 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_System_Numerics_Quaternion {
    pub f_X: f32,
    pub f_Y: f32,
    pub f_Z: f32,
    pub f_W: f32,
}
const _: () = assert!(core::mem::size_of::<S_System_Numerics_Quaternion>() == 16);

/// Rotation, 16 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_Rotation {
    pub f__quat: S_System_Numerics_Quaternion,
}
const _: () = assert!(core::mem::size_of::<S_Rotation>() == 16);

/// audio_source_indexstate_t, 16 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_audio_source_indexstate_t {
    pub f_m_nPacketIndex: u32,
    pub f_m_nBufferSampleOffset: u32,
    pub f_m_nSampleFracOffset: u32,
    pub f_m_nDelaySamples: u32,
}
const _: () = assert!(core::mem::size_of::<S_audio_source_indexstate_t>() == 16);

/// Sandbox.StringToken, 4 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_Sandbox_StringToken {
    pub f_Value: u32,
}
const _: () = assert!(core::mem::size_of::<S_Sandbox_StringToken>() == 4);

/// System.Numerics.Matrix4x4, 64 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_System_Numerics_Matrix4x4 {
    pub f_M11: f32,
    pub f_M12: f32,
    pub f_M13: f32,
    pub f_M14: f32,
    pub f_M21: f32,
    pub f_M22: f32,
    pub f_M23: f32,
    pub f_M24: f32,
    pub f_M31: f32,
    pub f_M32: f32,
    pub f_M33: f32,
    pub f_M34: f32,
    pub f_M41: f32,
    pub f_M42: f32,
    pub f_M43: f32,
    pub f_M44: f32,
}
const _: () = assert!(core::mem::size_of::<S_System_Numerics_Matrix4x4>() == 64);

/// Matrix, 64 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_Matrix {
    pub f__numerics: S_System_Numerics_Matrix4x4,
}
const _: () = assert!(core::mem::size_of::<S_Matrix>() == 64);

/// System.Numerics.Vector4, 16 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_System_Numerics_Vector4 {
    pub f_X: f32,
    pub f_Y: f32,
    pub f_Z: f32,
    pub f_W: f32,
}
const _: () = assert!(core::mem::size_of::<S_System_Numerics_Vector4>() == 16);

/// Vector4, 16 bytes, Pack 16, explicit
#[repr(C, packed)] #[derive(Clone, Copy)]
pub struct S_Vector4 {
    pub f__vec: S_System_Numerics_Vector4,
}
const _: () = assert!(core::mem::size_of::<S_Vector4>() == 16);

/// Angles, 12 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_Angles {
    pub f_pitch: f32,
    pub f_yaw: f32,
    pub f_roll: f32,
}
const _: () = assert!(core::mem::size_of::<S_Angles>() == 12);

/// NativeRect, 16 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_NativeRect {
    pub f_x: i32,
    pub f_y: i32,
    pub f_w: i32,
    pub f_h: i32,
}
const _: () = assert!(core::mem::size_of::<S_NativeRect>() == 16);

/// Transform, 40 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_Transform {
    pub f_Position: S_Vector3,
    pub f_Scale: S_Vector3,
    pub f_Rotation: S_Rotation,
}
const _: () = assert!(core::mem::size_of::<S_Transform>() == 40);

/// BBox, 24 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_BBox {
    pub f_Mins: S_Vector3,
    pub f_Maxs: S_Vector3,
}
const _: () = assert!(core::mem::size_of::<S_BBox>() == 24);

/// Color32, 4 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_Color32 {
    pub f_r: u8,
    pub f_g: u8,
    pub f_b: u8,
    pub f_a: u8,
}
const _: () = assert!(core::mem::size_of::<S_Color32>() == 4);

/// System.Guid, 16 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_System_Guid {
    pub f__a: i32,
    pub f__b: i16,
    pub f__c: i16,
    pub f__d: u8,
    pub f__e: u8,
    pub f__f: u8,
    pub f__g: u8,
    pub f__h: u8,
    pub f__i: u8,
    pub f__j: u8,
    pub f__k: u8,
}
const _: () = assert!(core::mem::size_of::<S_System_Guid>() == 16);

/// System.Numerics.Vector2, 8 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_System_Numerics_Vector2 {
    pub f_X: f32,
    pub f_Y: f32,
}
const _: () = assert!(core::mem::size_of::<S_System_Numerics_Vector2>() == 8);

/// Vector2, 8 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_Vector2 {
    pub f__vec: S_System_Numerics_Vector2,
}
const _: () = assert!(core::mem::size_of::<S_Vector2>() == 8);

/// NativeEngine.RenderViewport, 28 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_NativeEngine_RenderViewport {
    pub f__m_nVersion_k__BackingField: i32,
    pub f__m_nTopLeftX_k__BackingField: i32,
    pub f__m_nTopLeftY_k__BackingField: i32,
    pub f__m_nWidth_k__BackingField: i32,
    pub f__m_nHeight_k__BackingField: i32,
    pub f__MinZ_k__BackingField: f32,
    pub f__MaxZ_k__BackingField: f32,
}
const _: () = assert!(core::mem::size_of::<S_NativeEngine_RenderViewport>() == 28);

/// MeshTraceOutput, 120 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_MeshTraceOutput {
    pub f_distance: f32,
    pub f_position: S_Vector3,
    pub f_normal: S_Vector3,
    pub f_material: usize,
    pub f_transform: S_Transform,
    pub f_sceneobjectHandle: i32,
    pub f_triangleIndex: i32,
    pub f_uv: S_Vector2,
    pub f_v0: usize,
    pub f_v1: usize,
    pub f_v2: usize,
}
const _: () = assert!(core::mem::size_of::<S_MeshTraceOutput>() == 120);

/// PhysicsTrace+Result, 136 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_PhysicsTrace_Result {
    pub f_StartPos: S_Vector3,
    pub f_EndPos: S_Vector3,
    pub f_HitPos: S_Vector3,
    pub f_Normal: S_Vector3,
    pub f_Fraction: f32,
    pub f_StartedInSolid: u8,
    pub f_PhysicsBodyHandle: i32,
    pub f_PhysicsShapeHandle: i32,
    pub f_SurfaceProperty: i32,
    pub f_TriangleIndex: i32,
    pub f_Tags: [u32; 16],
}
const _: () = assert!(core::mem::size_of::<S_PhysicsTrace_Result>() == 136);

/// Sandbox.Network.HSteamListenSocket, 4 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_Sandbox_Network_HSteamListenSocket {
    pub f_Id: u32,
}
const _: () = assert!(core::mem::size_of::<S_Sandbox_Network_HSteamListenSocket>() == 4);

/// Sandbox.Network.HSteamNetPollGroup, 4 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_Sandbox_Network_HSteamNetPollGroup {
    pub f_Id: u32,
}
const _: () = assert!(core::mem::size_of::<S_Sandbox_Network_HSteamNetPollGroup>() == 4);

/// Sandbox.Network.HSteamNetConnection, 4 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_Sandbox_Network_HSteamNetConnection {
    pub f_Id: u32,
}
const _: () = assert!(core::mem::size_of::<S_Sandbox_Network_HSteamNetConnection>() == 4);

/// NativeEngine.AnimVariant, 17 bytes, Pack 1, explicit
#[repr(C, packed)] #[derive(Clone, Copy)]
pub struct S_NativeEngine_AnimVariant {
    pub f__rawValue: S_Vector4,
    pub f_Type: u8,
}
const _: () = assert!(core::mem::size_of::<S_NativeEngine_AnimVariant>() == 17);

/// Sandbox.Sphere, 16 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_Sandbox_Sphere {
    pub f_Center: S_Vector3,
    pub f_Radius: f32,
}
const _: () = assert!(core::mem::size_of::<S_Sandbox_Sphere>() == 16);

/// Capsule, 28 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_Capsule {
    pub f_CenterA: S_Vector3,
    pub f_CenterB: S_Vector3,
    pub f_Radius: f32,
}
const _: () = assert!(core::mem::size_of::<S_Capsule>() == 28);

/// Steamworks.Data.HAuthTicket, 4 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_Steamworks_Data_HAuthTicket {
    pub f_Value: u32,
}
const _: () = assert!(core::mem::size_of::<S_Steamworks_Data_HAuthTicket>() == 4);

/// Steamworks.Data.SteamNetConnectionRealTimeStatus_t, 120 bytes, Pack 4
#[repr(C, packed(4))] #[derive(Clone, Copy)]
pub struct S_Steamworks_Data_SteamNetConnectionRealTimeStatus_t {
    pub f_m_eState: i32,
    pub f_m_nPing: i32,
    pub f_m_flConnectionQualityLocal: f32,
    pub f_m_flConnectionQualityRemote: f32,
    pub f_m_flOutPacketsPerSec: f32,
    pub f_m_flOutBytesPerSec: f32,
    pub f_m_flInPacketsPerSec: f32,
    pub f_m_flInBytesPerSec: f32,
    pub f_m_nSendRateBytesPerSecond: i32,
    pub f_m_cbPendingUnreliable: i32,
    pub f_m_cbPendingReliable: i32,
    pub f_m_cbSentUnackedReliable: i32,
    pub f_m_usecQueueTime: i64,
    pub f_m_usecMaxJitter: i32,
    pub f_reserved: [u32; 15],
}
const _: () = assert!(core::mem::size_of::<S_Steamworks_Data_SteamNetConnectionRealTimeStatus_t>() == 120);

/// SheetFrameImage_t, 32 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_SheetFrameImage_t {
    pub f_uv1: S_Vector4,
    pub f_uv2: S_Vector4,
}
const _: () = assert!(core::mem::size_of::<S_SheetFrameImage_t>() == 32);

/// Facepunch.XR.InstanceProperties, 257 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_Facepunch_XR_InstanceProperties {
    pub f_systemName: [u8; 256],
    pub f_supportsHandTracking: u8,
}
const _: () = assert!(core::mem::size_of::<S_Facepunch_XR_InstanceProperties>() == 257);

/// RenderDisplayMode_t, 20 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_RenderDisplayMode_t {
    pub f_m_nWidth: i32,
    pub f_m_nHeight: i32,
    pub f_m_Format: i32,
    pub f_m_nRefreshRateNumerator: i32,
    pub f_m_nRefreshRateDenominator: i32,
}
const _: () = assert!(core::mem::size_of::<S_RenderDisplayMode_t>() == 20);

/// RenderDeviceInfo_t, 28 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_RenderDeviceInfo_t {
    pub f_m_DisplayMode: S_RenderDisplayMode_t,
    pub f_m_nMultisampleType: i32,
    pub f_m_nModeUsage: u8,
    pub f_m_bWaitForVSync: u8,
    pub f_m_bIsMainWindow: u8,
}
const _: () = assert!(core::mem::size_of::<S_RenderDeviceInfo_t>() == 28);

/// NativeEngine.AudioDeviceDesc, 2 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_NativeEngine_AudioDeviceDesc {
    pub f_IsAvailable: u8,
    pub f_IsDefault: u8,
}
const _: () = assert!(core::mem::size_of::<S_NativeEngine_AudioDeviceDesc>() == 2);

/// b2WorldId, 4 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2WorldId {
    pub f_index1: u16,
    pub f_generation: u16,
}
const _: () = assert!(core::mem::size_of::<S_b2WorldId>() == 4);

/// b2BodyId, 8 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2BodyId {
    pub f_index1: i32,
    pub f_world0: u16,
    pub f_generation: u16,
}
const _: () = assert!(core::mem::size_of::<S_b2BodyId>() == 8);

/// b2ShapeId, 8 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2ShapeId {
    pub f_index1: i32,
    pub f_world0: u16,
    pub f_generation: u16,
}
const _: () = assert!(core::mem::size_of::<S_b2ShapeId>() == 8);

/// b2ChainId, 8 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2ChainId {
    pub f_index1: i32,
    pub f_world0: u16,
    pub f_generation: u16,
}
const _: () = assert!(core::mem::size_of::<S_b2ChainId>() == 8);

/// b2JointId, 8 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2JointId {
    pub f_index1: i32,
    pub f_world0: u16,
    pub f_generation: u16,
}
const _: () = assert!(core::mem::size_of::<S_b2JointId>() == 8);

/// b2BodyEvents, 16 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2BodyEvents {
    pub f_moveEvents: usize,
    pub f_moveCount: i32,
}
const _: () = assert!(core::mem::size_of::<S_b2BodyEvents>() == 16);

/// b2SensorEvents, 24 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2SensorEvents {
    pub f_beginEvents: usize,
    pub f_endEvents: usize,
    pub f_beginCount: i32,
    pub f_endCount: i32,
}
const _: () = assert!(core::mem::size_of::<S_b2SensorEvents>() == 24);

/// b2ContactEvents, 40 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2ContactEvents {
    pub f_beginEvents: usize,
    pub f_endEvents: usize,
    pub f_hitEvents: usize,
    pub f_beginCount: i32,
    pub f_endCount: i32,
    pub f_hitCount: i32,
}
const _: () = assert!(core::mem::size_of::<S_b2ContactEvents>() == 40);

/// b2JointEvents, 16 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2JointEvents {
    pub f_jointEvents: usize,
    pub f_count: i32,
}
const _: () = assert!(core::mem::size_of::<S_b2JointEvents>() == 16);

/// b2TreeStats, 8 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2TreeStats {
    pub f_nodeVisits: i32,
    pub f_leafVisits: i32,
}
const _: () = assert!(core::mem::size_of::<S_b2TreeStats>() == 8);

/// b2Vec2, 8 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2Vec2 {
    pub f_x: f32,
    pub f_y: f32,
}
const _: () = assert!(core::mem::size_of::<S_b2Vec2>() == 8);

/// b2RayResult, 40 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2RayResult {
    pub f_shapeId: S_b2ShapeId,
    pub f_point: S_b2Vec2,
    pub f_normal: S_b2Vec2,
    pub f_fraction: f32,
    pub f_nodeVisits: i32,
    pub f_leafVisits: i32,
    pub f_hit: u8,
}
const _: () = assert!(core::mem::size_of::<S_b2RayResult>() == 40);

/// b2Profile, 92 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2Profile {
    pub f_step: f32,
    pub f_pairs: f32,
    pub f_collide: f32,
    pub f_solve: f32,
    pub f_solverSetup: f32,
    pub f_constraints: f32,
    pub f_prepareConstraints: f32,
    pub f_integrateVelocities: f32,
    pub f_warmStart: f32,
    pub f_solveImpulses: f32,
    pub f_integratePositions: f32,
    pub f_relaxImpulses: f32,
    pub f_applyRestitution: f32,
    pub f_storeImpulses: f32,
    pub f_splitIslands: f32,
    pub f_transforms: f32,
    pub f_sensorHits: f32,
    pub f_jointEvents: f32,
    pub f_hitEvents: f32,
    pub f_refit: f32,
    pub f_bullets: f32,
    pub f_sleepIslands: f32,
    pub f_sensors: f32,
}
const _: () = assert!(core::mem::size_of::<S_b2Profile>() == 92);

/// b2Counters, 144 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2Counters {
    pub f_bodyCount: i32,
    pub f_shapeCount: i32,
    pub f_contactCount: i32,
    pub f_jointCount: i32,
    pub f_islandCount: i32,
    pub f_stackUsed: i32,
    pub f_staticTreeHeight: i32,
    pub f_treeHeight: i32,
    pub f_byteCount: i32,
    pub f_taskCount: i32,
    pub f_colorCount0: i32,
    pub f_colorCount1: i32,
    pub f_colorCount2: i32,
    pub f_colorCount3: i32,
    pub f_colorCount4: i32,
    pub f_colorCount5: i32,
    pub f_colorCount6: i32,
    pub f_colorCount7: i32,
    pub f_colorCount8: i32,
    pub f_colorCount9: i32,
    pub f_colorCount10: i32,
    pub f_colorCount11: i32,
    pub f_colorCount12: i32,
    pub f_colorCount13: i32,
    pub f_colorCount14: i32,
    pub f_colorCount15: i32,
    pub f_colorCount16: i32,
    pub f_colorCount17: i32,
    pub f_colorCount18: i32,
    pub f_colorCount19: i32,
    pub f_colorCount20: i32,
    pub f_colorCount21: i32,
    pub f_colorCount22: i32,
    pub f_colorCount23: i32,
    pub f_awakeContactCount: i32,
    pub f_recycledContactCount: i32,
}
const _: () = assert!(core::mem::size_of::<S_b2Counters>() == 144);

/// b2Capacity, 20 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2Capacity {
    pub f_staticShapeCount: i32,
    pub f_dynamicShapeCount: i32,
    pub f_staticBodyCount: i32,
    pub f_dynamicBodyCount: i32,
    pub f_contactCount: i32,
}
const _: () = assert!(core::mem::size_of::<S_b2Capacity>() == 20);

/// b2Rot, 8 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2Rot {
    pub f_c: f32,
    pub f_s: f32,
}
const _: () = assert!(core::mem::size_of::<S_b2Rot>() == 8);

/// b2Transform, 16 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2Transform {
    pub f_p: S_b2Vec2,
    pub f_q: S_b2Rot,
}
const _: () = assert!(core::mem::size_of::<S_b2Transform>() == 16);

/// b2MassData, 16 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2MassData {
    pub f_mass: f32,
    pub f_center: S_b2Vec2,
    pub f_rotationalInertia: f32,
}
const _: () = assert!(core::mem::size_of::<S_b2MassData>() == 16);

/// b2MotionLocks, 3 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2MotionLocks {
    pub f_linearX: u8,
    pub f_linearY: u8,
    pub f_angularZ: u8,
}
const _: () = assert!(core::mem::size_of::<S_b2MotionLocks>() == 3);

/// b2AABB, 16 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2AABB {
    pub f_lowerBound: S_b2Vec2,
    pub f_upperBound: S_b2Vec2,
}
const _: () = assert!(core::mem::size_of::<S_b2AABB>() == 16);

/// b2SurfaceMaterial, 32 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2SurfaceMaterial {
    pub f_friction: f32,
    pub f_restitution: f32,
    pub f_rollingResistance: f32,
    pub f_tangentSpeed: f32,
    pub f_userMaterialId: u64,
    pub f_customColor: u32,
}
const _: () = assert!(core::mem::size_of::<S_b2SurfaceMaterial>() == 32);

/// b2Filter, 24 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2Filter {
    pub f_categoryBits: u64,
    pub f_maskBits: u64,
    pub f_groupIndex: i32,
}
const _: () = assert!(core::mem::size_of::<S_b2Filter>() == 24);

/// b2CastOutput, 28 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2CastOutput {
    pub f_normal: S_b2Vec2,
    pub f_point: S_b2Vec2,
    pub f_fraction: f32,
    pub f_iterations: i32,
    pub f_hit: u8,
}
const _: () = assert!(core::mem::size_of::<S_b2CastOutput>() == 28);

/// b2Circle, 12 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2Circle {
    pub f_center: S_b2Vec2,
    pub f_radius: f32,
}
const _: () = assert!(core::mem::size_of::<S_b2Circle>() == 12);

/// b2Segment, 16 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2Segment {
    pub f_point1: S_b2Vec2,
    pub f_point2: S_b2Vec2,
}
const _: () = assert!(core::mem::size_of::<S_b2Segment>() == 16);

/// b2ChainSegment, 36 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2ChainSegment {
    pub f_ghost1: S_b2Vec2,
    pub f_segment: S_b2Segment,
    pub f_ghost2: S_b2Vec2,
    pub f_chainId: i32,
}
const _: () = assert!(core::mem::size_of::<S_b2ChainSegment>() == 36);

/// b2Capsule, 20 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2Capsule {
    pub f_center1: S_b2Vec2,
    pub f_center2: S_b2Vec2,
    pub f_radius: f32,
}
const _: () = assert!(core::mem::size_of::<S_b2Capsule>() == 20);

/// b2Polygon, 144 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2Polygon {
    pub f_v0: S_b2Vec2,
    pub f_v1: S_b2Vec2,
    pub f_v2: S_b2Vec2,
    pub f_v3: S_b2Vec2,
    pub f_v4: S_b2Vec2,
    pub f_v5: S_b2Vec2,
    pub f_v6: S_b2Vec2,
    pub f_v7: S_b2Vec2,
    pub f_n0: S_b2Vec2,
    pub f_n1: S_b2Vec2,
    pub f_n2: S_b2Vec2,
    pub f_n3: S_b2Vec2,
    pub f_n4: S_b2Vec2,
    pub f_n5: S_b2Vec2,
    pub f_n6: S_b2Vec2,
    pub f_n7: S_b2Vec2,
    pub f_centroid: S_b2Vec2,
    pub f_radius: f32,
    pub f_count: i32,
}
const _: () = assert!(core::mem::size_of::<S_b2Polygon>() == 144);

/// b2ContactId, 12 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2ContactId {
    pub f_index1: i32,
    pub f_world0: u16,
    pub f_padding: i16,
    pub f_generation: u32,
}
const _: () = assert!(core::mem::size_of::<S_b2ContactId>() == 12);

/// b2ManifoldPoint, 52 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2ManifoldPoint {
    pub f_clipPoint: S_b2Vec2,
    pub f_anchorA: S_b2Vec2,
    pub f_anchorB: S_b2Vec2,
    pub f_separation: f32,
    pub f_baseSeparation: f32,
    pub f_normalImpulse: f32,
    pub f_tangentImpulse: f32,
    pub f_totalNormalImpulse: f32,
    pub f_normalVelocity: f32,
    pub f_id: u16,
    pub f_persisted: u8,
}
const _: () = assert!(core::mem::size_of::<S_b2ManifoldPoint>() == 52);

/// b2Manifold, 120 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2Manifold {
    pub f_normal: S_b2Vec2,
    pub f_rollingImpulse: f32,
    pub f_p0: S_b2ManifoldPoint,
    pub f_p1: S_b2ManifoldPoint,
    pub f_pointCount: i32,
}
const _: () = assert!(core::mem::size_of::<S_b2Manifold>() == 120);

/// b2ContactData, 148 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2ContactData {
    pub f_contactId: S_b2ContactId,
    pub f_shapeIdA: S_b2ShapeId,
    pub f_shapeIdB: S_b2ShapeId,
    pub f_manifold: S_b2Manifold,
}
const _: () = assert!(core::mem::size_of::<S_b2ContactData>() == 148);

/// b2WorldDef, 112 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2WorldDef {
    pub f_gravity: S_b2Vec2,
    pub f_restitutionThreshold: f32,
    pub f_hitEventThreshold: f32,
    pub f_contactHertz: f32,
    pub f_contactDampingRatio: f32,
    pub f_contactSpeed: f32,
    pub f_maximumLinearSpeed: f32,
    pub f_frictionCallback: usize,
    pub f_restitutionCallback: usize,
    pub f_enableSleep: u8,
    pub f_enableContinuous: u8,
    pub f_enableContactSoftening: u8,
    pub f_workerCount: i32,
    pub f_enqueueTask: usize,
    pub f_finishTask: usize,
    pub f_userTaskContext: usize,
    pub f_userData: usize,
    pub f_capacity: S_b2Capacity,
    pub f_internalValue: i32,
}
const _: () = assert!(core::mem::size_of::<S_b2WorldDef>() == 112);

/// b2BodyDef, 80 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2BodyDef {
    pub f_type: i32,
    pub f_position: S_b2Vec2,
    pub f_rotation: S_b2Rot,
    pub f_linearVelocity: S_b2Vec2,
    pub f_angularVelocity: f32,
    pub f_linearDamping: f32,
    pub f_angularDamping: f32,
    pub f_gravityScale: f32,
    pub f_sleepThreshold: f32,
    pub f_name: usize,
    pub f_userData: usize,
    pub f_motionLocks: S_b2MotionLocks,
    pub f_enableSleep: u8,
    pub f_isAwake: u8,
    pub f_isBullet: u8,
    pub f_isEnabled: u8,
    pub f_allowFastRotation: u8,
    pub f_internalValue: i32,
}
const _: () = assert!(core::mem::size_of::<S_b2BodyDef>() == 80);

/// b2ShapeDef, 88 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2ShapeDef {
    pub f_userData: usize,
    pub f_material: S_b2SurfaceMaterial,
    pub f_density: f32,
    pub f_filter: S_b2Filter,
    pub f_enableCustomFiltering: u8,
    pub f_isSensor: u8,
    pub f_enableSensorEvents: u8,
    pub f_enableContactEvents: u8,
    pub f_enableHitEvents: u8,
    pub f_enablePreSolveEvents: u8,
    pub f_invokeContactCreation: u8,
    pub f_updateBodyMass: u8,
    pub f_internalValue: i32,
}
const _: () = assert!(core::mem::size_of::<S_b2ShapeDef>() == 88);

/// b2ChainDef, 72 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2ChainDef {
    pub f_userData: usize,
    pub f_points: usize,
    pub f_count: i32,
    pub f_materials: usize,
    pub f_materialCount: i32,
    pub f_filter: S_b2Filter,
    pub f_isLoop: u8,
    pub f_enableSensorEvents: u8,
    pub f_internalValue: i32,
}
const _: () = assert!(core::mem::size_of::<S_b2ChainDef>() == 72);

/// b2JointDef, 80 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2JointDef {
    pub f_userData: usize,
    pub f_bodyIdA: S_b2BodyId,
    pub f_bodyIdB: S_b2BodyId,
    pub f_localFrameA: S_b2Transform,
    pub f_localFrameB: S_b2Transform,
    pub f_forceThreshold: f32,
    pub f_torqueThreshold: f32,
    pub f_constraintHertz: f32,
    pub f_constraintDampingRatio: f32,
    pub f_drawScale: f32,
    pub f_collideConnected: u8,
}
const _: () = assert!(core::mem::size_of::<S_b2JointDef>() == 80);

/// b2DistanceJointDef, 136 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2DistanceJointDef {
    pub f_baseDef: S_b2JointDef,
    pub f_length: f32,
    pub f_enableSpring: u8,
    pub f_lowerSpringForce: f32,
    pub f_upperSpringForce: f32,
    pub f_hertz: f32,
    pub f_dampingRatio: f32,
    pub f_enableLimit: u8,
    pub f_minLength: f32,
    pub f_maxLength: f32,
    pub f_enableMotor: u8,
    pub f_maxMotorForce: f32,
    pub f_motorSpeed: f32,
    pub f_internalValue: i32,
}
const _: () = assert!(core::mem::size_of::<S_b2DistanceJointDef>() == 136);

/// b2MotorJointDef, 128 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2MotorJointDef {
    pub f_baseDef: S_b2JointDef,
    pub f_linearVelocity: S_b2Vec2,
    pub f_maxVelocityForce: f32,
    pub f_angularVelocity: f32,
    pub f_maxVelocityTorque: f32,
    pub f_linearHertz: f32,
    pub f_linearDampingRatio: f32,
    pub f_maxSpringForce: f32,
    pub f_angularHertz: f32,
    pub f_angularDampingRatio: f32,
    pub f_maxSpringTorque: f32,
    pub f_internalValue: i32,
}
const _: () = assert!(core::mem::size_of::<S_b2MotorJointDef>() == 128);

/// b2FilterJointDef, 88 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2FilterJointDef {
    pub f_baseDef: S_b2JointDef,
    pub f_internalValue: i32,
}
const _: () = assert!(core::mem::size_of::<S_b2FilterJointDef>() == 88);

/// b2PrismaticJointDef, 128 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2PrismaticJointDef {
    pub f_baseDef: S_b2JointDef,
    pub f_enableSpring: u8,
    pub f_hertz: f32,
    pub f_dampingRatio: f32,
    pub f_targetTranslation: f32,
    pub f_enableLimit: u8,
    pub f_lowerTranslation: f32,
    pub f_upperTranslation: f32,
    pub f_enableMotor: u8,
    pub f_maxMotorForce: f32,
    pub f_motorSpeed: f32,
    pub f_internalValue: i32,
}
const _: () = assert!(core::mem::size_of::<S_b2PrismaticJointDef>() == 128);

/// b2RevoluteJointDef, 128 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2RevoluteJointDef {
    pub f_baseDef: S_b2JointDef,
    pub f_targetAngle: f32,
    pub f_enableSpring: u8,
    pub f_hertz: f32,
    pub f_dampingRatio: f32,
    pub f_enableLimit: u8,
    pub f_lowerAngle: f32,
    pub f_upperAngle: f32,
    pub f_enableMotor: u8,
    pub f_maxMotorTorque: f32,
    pub f_motorSpeed: f32,
    pub f_internalValue: i32,
}
const _: () = assert!(core::mem::size_of::<S_b2RevoluteJointDef>() == 128);

/// b2WeldJointDef, 104 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2WeldJointDef {
    pub f_baseDef: S_b2JointDef,
    pub f_linearHertz: f32,
    pub f_angularHertz: f32,
    pub f_linearDampingRatio: f32,
    pub f_angularDampingRatio: f32,
    pub f_internalValue: i32,
}
const _: () = assert!(core::mem::size_of::<S_b2WeldJointDef>() == 104);

/// b2WheelJointDef, 120 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2WheelJointDef {
    pub f_baseDef: S_b2JointDef,
    pub f_enableSpring: u8,
    pub f_hertz: f32,
    pub f_dampingRatio: f32,
    pub f_enableLimit: u8,
    pub f_lowerTranslation: f32,
    pub f_upperTranslation: f32,
    pub f_enableMotor: u8,
    pub f_maxMotorTorque: f32,
    pub f_motorSpeed: f32,
    pub f_internalValue: i32,
}
const _: () = assert!(core::mem::size_of::<S_b2WheelJointDef>() == 120);

/// b2ExplosionDef, 32 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2ExplosionDef {
    pub f_maskBits: u64,
    pub f_position: S_b2Vec2,
    pub f_radius: f32,
    pub f_falloff: f32,
    pub f_impulsePerLength: f32,
}
const _: () = assert!(core::mem::size_of::<S_b2ExplosionDef>() == 32);

/// b2QueryFilter, 16 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2QueryFilter {
    pub f_categoryBits: u64,
    pub f_maskBits: u64,
}
const _: () = assert!(core::mem::size_of::<S_b2QueryFilter>() == 16);

/// b2Hull, 68 bytes
#[repr(C)] #[derive(Clone, Copy)]
pub struct S_b2Hull {
    pub f_p0: S_b2Vec2,
    pub f_p1: S_b2Vec2,
    pub f_p2: S_b2Vec2,
    pub f_p3: S_b2Vec2,
    pub f_p4: S_b2Vec2,
    pub f_p5: S_b2Vec2,
    pub f_p6: S_b2Vec2,
    pub f_p7: S_b2Vec2,
    pub f_count: i32,
}
const _: () = assert!(core::mem::size_of::<S_b2Hull>() == 68);

unsafe extern "C" fn s1(_a0: usize, _a1: usize, _a2: usize, _a3: i32, _a4: i32) -> i32 { hit(1); crate::handles::create(1, "SceneObject") }
unsafe extern "C" fn s2() -> i32 { hit(2); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3(_a0: usize) { hit(3);  }
unsafe extern "C" fn s4() -> usize { hit(4); crate::trace::object(4) }
unsafe extern "C" fn s5(_a0: usize) -> i32 { hit(5); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s6(_a0: usize, _a1: i32, _a2: usize, _a3: i32) { hit(6);  }
unsafe extern "C" fn s7(_a0: usize, _a1: i32, _a2: usize) { hit(7);  }
unsafe extern "C" fn s8(_a0: usize, _a1: i32, _a2: f32) { hit(8);  }
unsafe extern "C" fn s9(_a0: usize, _a1: i32, _a2: i32) { hit(9);  }
unsafe extern "C" fn s10(_a0: usize, _a1: i32, _a2: i32) { hit(10);  }
unsafe extern "C" fn s11(_a0: usize, _a1: i32, _a2: i32) { hit(11);  }
unsafe extern "C" fn s12(_a0: usize, _a1: i32) -> usize { hit(12); crate::trace::object(12) }
unsafe extern "C" fn s13(_a0: usize, _a1: i32) -> S_Vector3 { hit(13); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s14(_a0: usize, _a1: i32) -> S_Rotation { hit(14); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s15(_a0: usize) -> usize { hit(15); crate::trace::object(15) }
unsafe extern "C" fn s16(_a0: usize, _a1: usize) { hit(16);  }
unsafe extern "C" fn s17(_a0: usize) -> u8 { hit(17); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s18(_a0: usize, _a1: u8) { hit(18);  }
unsafe extern "C" fn s19(_a0: usize) -> i32 { hit(19); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s20(_a0: usize, _a1: i32) { hit(20);  }
unsafe extern "C" fn s21() -> usize { hit(21); crate::trace::object(21) }
unsafe extern "C" fn s22(_a0: usize) { hit(22);  }
unsafe extern "C" fn s23(_a0: usize) -> usize { hit(23); crate::trace::object(23) }
unsafe extern "C" fn s24(_a0: usize) { hit(24);  }
unsafe extern "C" fn s25(_a0: usize) -> f32 { hit(25); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s26(_a0: usize) -> f32 { hit(26); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s27(_a0: usize, _a1: f32, _a2: f32) { hit(27);  }
unsafe extern "C" fn s28(_a0: usize, _a1: usize) { hit(28);  }
unsafe extern "C" fn s29(_a0: usize, _a1: usize, _a2: f32) { hit(29);  }
unsafe extern "C" fn s30(_a0: usize, _a1: usize, _a2: f32, _a3: f32) { hit(30);  }
unsafe extern "C" fn s31(_a0: i32) -> usize { hit(31); crate::trace::object(31) }
unsafe extern "C" fn s32(_a0: usize) { hit(32);  }
unsafe extern "C" fn s33(_a0: usize, _a1: i32) -> usize { hit(33); crate::trace::object(33) }
unsafe extern "C" fn s34(_a0: usize) { hit(34);  }
unsafe extern "C" fn s35(_a0: usize) -> i32 { hit(35); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s36(_a0: usize) -> i32 { hit(36); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s37(_a0: usize, _a1: i32) { hit(37);  }
unsafe extern "C" fn s38(_a0: usize, _a1: u32) { hit(38);  }
unsafe extern "C" fn s39(_a0: usize, _a1: i32) { hit(39);  }
unsafe extern "C" fn s40(_a0: usize) -> i32 { hit(40); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s41(_a0: usize) -> i32 { hit(41); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s42(_a0: usize, _a1: i32) { hit(42);  }
unsafe extern "C" fn s43(_a0: usize, _a1: usize) { hit(43);  }
unsafe extern "C" fn s44(_a0: usize) -> S_audio_source_indexstate_t { hit(44); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s45(_a0: usize) -> usize { hit(45); crate::trace::object(45) }
unsafe extern "C" fn s46(_a0: usize) -> i32 { hit(46); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s47(_a0: usize) -> i32 { hit(47); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s48(_a0: usize, _a1: f32) -> i32 { hit(48); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s49(_a0: usize, _a1: i32) { hit(49);  }
unsafe extern "C" fn s50(_a0: usize, _a1: f32, _a2: usize) { hit(50);  }
unsafe extern "C" fn s51(_a0: usize) { hit(51);  }
unsafe extern "C" fn s52(_a0: usize, _a1: usize, _a2: usize, _a3: i32) { hit(52);  }
unsafe extern "C" fn s53(_a0: usize, _a1: S_Sandbox_StringToken, _a2: f32) -> i32 { hit(53); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s54(_a0: usize, _a1: S_Sandbox_StringToken, _a2: S_Sandbox_StringToken) -> i32 { hit(54); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s55(_a0: i32) -> usize { hit(55); crate::trace::object(55) }
unsafe extern "C" fn s56(_a0: i32) -> usize { hit(56); crate::trace::object(56) }
unsafe extern "C" fn s57(_a0: i32, _a1: u32) -> i32 { hit(57); crate::handles::create(57, "AudioStream") }
unsafe extern "C" fn s58(_a0: usize) { hit(58);  }
unsafe extern "C" fn s59(_a0: usize, _a1: usize, _a2: u32, _a3: u32) { hit(59);  }
unsafe extern "C" fn s60(_a0: usize) -> u32 { hit(60); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s61(_a0: usize) -> u32 { hit(61); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s62(_a0: usize) -> u32 { hit(62); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s63(_a0: usize) { hit(63);  }
unsafe extern "C" fn s64(_a0: usize) { hit(64);  }
unsafe extern "C" fn s65(_a0: usize) { hit(65);  }
unsafe extern "C" fn s66(_a0: usize) -> usize { hit(66); crate::trace::object(66) }
unsafe extern "C" fn s67(_a0: usize) -> usize { hit(67); crate::trace::object(67) }
unsafe extern "C" fn s68() -> usize { hit(68); crate::trace::object(68) }
unsafe extern "C" fn s69(_a0: usize) { hit(69);  }
unsafe extern "C" fn s70(_a0: usize, _a1: usize, _a2: f32, _a3: i32, _a4: usize, _a5: usize) { hit(70);  }
unsafe extern "C" fn s71(_a0: usize, _a1: usize) { hit(71);  }
unsafe extern "C" fn s72(_a0: usize, _a1: usize) { hit(72);  }
unsafe extern "C" fn s73(_a0: usize, _a1: usize) { hit(73);  }
unsafe extern "C" fn s74(_a0: usize, _a1: usize, _a2: usize) { hit(74);  }
unsafe extern "C" fn s75(_a0: usize, _a1: usize, _a2: f32) { hit(75);  }
unsafe extern "C" fn s76(_a0: usize, _a1: usize, _a2: usize, _a3: f32) { hit(76);  }
unsafe extern "C" fn s77(_a0: usize, _a1: usize) { hit(77);  }
unsafe extern "C" fn s78(_a0: usize, _a1: usize) { hit(78);  }
unsafe extern "C" fn s79(_a0: usize) { hit(79);  }
unsafe extern "C" fn s80(_a0: usize, _a1: i32) -> usize { hit(80); crate::trace::object(80) }
unsafe extern "C" fn s81(_a0: usize) { hit(81);  }
unsafe extern "C" fn s82(_a0: usize, _a1: usize) { hit(82);  }
unsafe extern "C" fn s83(_a0: usize, _a1: usize) { hit(83);  }
unsafe extern "C" fn s84(_a0: usize, _a1: usize) { hit(84);  }
unsafe extern "C" fn s85(_a0: usize, _a1: usize, _a2: usize, _a3: usize) -> i32 { hit(85); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s86(_a0: usize, _a1: usize, _a2: i32) { hit(86);  }
unsafe extern "C" fn s87(_a0: usize, _a1: usize, _a2: i32, _a3: i32, _a4: i32) { hit(87);  }
unsafe extern "C" fn s88(_a0: usize) { hit(88);  }
unsafe extern "C" fn s89(_a0: usize, _a1: i32, _a2: i32, _a3: i32, _a4: i32) { hit(89);  }
unsafe extern "C" fn s90(_a0: usize, _a1: i32, _a2: i32) { hit(90);  }
unsafe extern "C" fn s91(_a0: usize, _a1: i32, _a2: i32) { hit(91);  }
unsafe extern "C" fn s92(_a0: usize) { hit(92);  }
unsafe extern "C" fn s93(_a0: usize) { hit(93);  }
unsafe extern "C" fn s94(_a0: usize, _a1: u32) { hit(94);  }
unsafe extern "C" fn s95(_a0: usize, _a1: u32) { hit(95);  }
unsafe extern "C" fn s96(_a0: usize) -> S_Matrix { hit(96); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s97(_a0: usize, _a1: usize) { hit(97);  }
unsafe extern "C" fn s98(_a0: usize) -> S_Vector4 { hit(98); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s99(_a0: usize, _a1: usize) { hit(99);  }
unsafe extern "C" fn s100(_a0: usize) -> S_Vector4 { hit(100); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s101(_a0: usize, _a1: usize) { hit(101);  }
unsafe extern "C" fn s102(_a0: usize) -> i32 { hit(102); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s103(_a0: usize, _a1: i32) { hit(103);  }
unsafe extern "C" fn s104(_a0: usize) -> S_Vector3 { hit(104); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s105(_a0: usize, _a1: usize) { hit(105);  }
unsafe extern "C" fn s106(_a0: usize) -> S_Angles { hit(106); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s107(_a0: usize, _a1: usize) { hit(107);  }
unsafe extern "C" fn s108(_a0: usize) -> f32 { hit(108); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s109(_a0: usize, _a1: f32) { hit(109);  }
unsafe extern "C" fn s110(_a0: usize) -> f32 { hit(110); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s111(_a0: usize, _a1: f32) { hit(111);  }
unsafe extern "C" fn s112(_a0: usize) -> f32 { hit(112); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s113(_a0: usize, _a1: f32) { hit(113);  }
unsafe extern "C" fn s114(_a0: usize) -> S_NativeRect { hit(114); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s115(_a0: usize, _a1: usize) { hit(115);  }
unsafe extern "C" fn s116(_a0: usize) -> S_Vector4 { hit(116); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s117(_a0: usize, _a1: usize) { hit(117);  }
unsafe extern "C" fn s118(_a0: usize) -> S_Vector4 { hit(118); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s119(_a0: usize, _a1: usize) { hit(119);  }
unsafe extern "C" fn s120(_a0: usize) -> i32 { hit(120); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s121(_a0: usize, _a1: i32) { hit(121);  }
unsafe extern "C" fn s122(_a0: usize) -> i32 { hit(122); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s123(_a0: usize, _a1: i32) { hit(123);  }
unsafe extern "C" fn s124(_a0: usize) -> i32 { hit(124); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s125(_a0: usize, _a1: i32) { hit(125);  }
unsafe extern "C" fn s126(_a0: usize) -> i32 { hit(126); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s127(_a0: usize, _a1: i32) { hit(127);  }
unsafe extern "C" fn s128(_a0: usize) -> i32 { hit(128); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s129(_a0: usize, _a1: i32) { hit(129);  }
unsafe extern "C" fn s130(_a0: usize) -> f32 { hit(130); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s131(_a0: usize, _a1: f32) { hit(131);  }
unsafe extern "C" fn s132(_a0: usize) -> i32 { hit(132); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s133(_a0: usize, _a1: i32) { hit(133);  }
unsafe extern "C" fn s134(_a0: usize) -> i64 { hit(134); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s135(_a0: usize, _a1: i64) { hit(135);  }
unsafe extern "C" fn s136(_a0: usize) -> i32 { hit(136); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s137(_a0: usize, _a1: i32) { hit(137);  }
unsafe extern "C" fn s138(_a0: usize) -> S_Vector3 { hit(138); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s139(_a0: usize, _a1: usize) { hit(139);  }
unsafe extern "C" fn s140(_a0: usize) -> S_Angles { hit(140); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s141(_a0: usize, _a1: usize) { hit(141);  }
unsafe extern "C" fn s142(_a0: usize) -> S_Matrix { hit(142); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s143(_a0: usize, _a1: usize) { hit(143);  }
unsafe extern "C" fn s144(_a0: usize) -> i32 { hit(144); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s145(_a0: usize, _a1: i32) { hit(145);  }
unsafe extern "C" fn s146(_a0: usize) -> i32 { hit(146); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s147(_a0: usize, _a1: i32) { hit(147);  }
unsafe extern "C" fn s148(_a0: usize) -> i32 { hit(148); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s149(_a0: usize, _a1: i32) { hit(149);  }
unsafe extern "C" fn s150(_a0: usize) -> usize { hit(150); _a0 }
unsafe extern "C" fn s151(_a0: usize) -> usize { hit(151); _a0 }
unsafe extern "C" fn s152(_a0: usize, _a1: i64, _a2: i64) { hit(152);  }
unsafe extern "C" fn s153(_a0: usize, _a1: i64) { hit(153);  }
unsafe extern "C" fn s154(_a0: usize, _a1: i64) -> i32 { hit(154); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s155(_a0: usize) -> i64 { hit(155); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s156(_a0: usize) -> i64 { hit(156); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s157(_a0: usize, _a1: i64) { hit(157);  }
unsafe extern "C" fn s158(_a0: usize, _a1: f32) { hit(158);  }
unsafe extern "C" fn s159(_a0: usize) { hit(159);  }
unsafe extern "C" fn s160(_a0: usize, _a1: usize, _a2: i32) { hit(160);  }
unsafe extern "C" fn s161(_a0: usize) -> S_Vector3 { hit(161); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s162(_a0: usize) -> i32 { hit(162); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s163(_a0: usize, _a1: usize) { hit(163);  }
unsafe extern "C" fn s164(_a0: usize) -> S_Vector4 { hit(164); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s165(_a0: usize, _a1: f32) { hit(165);  }
unsafe extern "C" fn s166(_a0: usize) -> f32 { hit(166); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s167(_a0: usize, _a1: usize) { hit(167);  }
unsafe extern "C" fn s168(_a0: usize) { hit(168);  }
unsafe extern "C" fn s169(_a0: usize, _a1: usize, _a2: S_Sandbox_StringToken, _a3: i32) { hit(169);  }
unsafe extern "C" fn s170(_a0: usize) -> i32 { hit(170); 1 }
unsafe extern "C" fn s171(_a0: usize) -> i32 { hit(171); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s172(_a0: usize) { hit(172);  }
unsafe extern "C" fn s173(_a0: usize) { hit(173);  }
unsafe extern "C" fn s174(_a0: usize) { hit(174);  }
unsafe extern "C" fn s175(_a0: usize) { hit(175);  }
unsafe extern "C" fn s176(_a0: usize, _a1: i32) { hit(176);  }
unsafe extern "C" fn s177(_a0: usize) -> f32 { hit(177); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s178(_a0: usize, _a1: usize) { hit(178);  }
unsafe extern "C" fn s179(_a0: usize) -> S_Transform { hit(179); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s180(_a0: usize, _a1: usize) { hit(180);  }
unsafe extern "C" fn s181(_a0: usize) -> S_BBox { hit(181); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s182(_a0: usize) { hit(182);  }
unsafe extern "C" fn s183(_a0: usize) -> i32 { hit(183); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s184(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize, _a3: u32) { hit(184);  }
unsafe extern "C" fn s185(_a0: usize, _a1: usize) { hit(185);  }
unsafe extern "C" fn s186(_a0: usize) -> usize { hit(186); crate::trace::object(186) }
unsafe extern "C" fn s187(_a0: usize, _a1: u64) { hit(187);  }
unsafe extern "C" fn s188(_a0: usize, _a1: u64) { hit(188);  }
unsafe extern "C" fn s189(_a0: usize, _a1: u64) { hit(189);  }
unsafe extern "C" fn s190(_a0: usize) -> u64 { hit(190); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s191(_a0: usize) -> i32 { hit(191); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s192(_a0: usize, _a1: i32) { hit(192);  }
unsafe extern "C" fn s193(_a0: usize) { hit(193);  }
unsafe extern "C" fn s194(_a0: usize) -> u64 { hit(194); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s195(_a0: usize) -> i32 { hit(195); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s196(_a0: usize) -> usize { hit(196); crate::trace::object(196) }
unsafe extern "C" fn s197(_a0: usize, _a1: S_Sandbox_StringToken) { hit(197);  }
unsafe extern "C" fn s198(_a0: usize, _a1: S_Sandbox_StringToken, _a2: i32) { hit(198);  }
unsafe extern "C" fn s199(_a0: usize, _a1: i32) { hit(199);  }
unsafe extern "C" fn s200(_a0: usize) -> i32 { hit(200); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s201(_a0: usize, _a1: i32) { hit(201);  }
unsafe extern "C" fn s202(_a0: usize, _a1: u32) { hit(202);  }
unsafe extern "C" fn s203(_a0: usize) { hit(203);  }
unsafe extern "C" fn s204(_a0: usize) -> i32 { hit(204); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s205(_a0: usize, _a1: i32) -> u32 { hit(205); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s206(_a0: usize, _a1: u32) { hit(206);  }
unsafe extern "C" fn s207(_a0: usize, _a1: u32) -> i32 { hit(207); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s208(_a0: usize, _a1: S_Sandbox_StringToken) { hit(208);  }
unsafe extern "C" fn s209(_a0: usize, _a1: S_Sandbox_StringToken) { hit(209);  }
unsafe extern "C" fn s210(_a0: usize) -> u32 { hit(210); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s211(_a0: usize, _a1: S_Sandbox_StringToken, _a2: i32) -> i32 { hit(211); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s212(_a0: usize) { hit(212);  }
unsafe extern "C" fn s213(_a0: usize, _a1: i32, _a2: usize) { hit(213);  }
unsafe extern "C" fn s214(_a0: usize) -> usize { hit(214); crate::trace::object(214) }
unsafe extern "C" fn s215(_a0: usize, _a1: usize) { hit(215);  }
unsafe extern "C" fn s216(_a0: usize) -> usize { hit(216); crate::trace::object(216) }
unsafe extern "C" fn s217(_a0: usize, _a1: usize) { hit(217);  }
unsafe extern "C" fn s218(_a0: usize) -> usize { hit(218); crate::trace::object(218) }
unsafe extern "C" fn s219(_a0: usize, _a1: usize) { hit(219);  }
unsafe extern "C" fn s220(_a0: usize) -> u32 { hit(220); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s221(_a0: usize, _a1: u32) { hit(221);  }
unsafe extern "C" fn s222(_a0: usize) -> u32 { hit(222); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s223(_a0: usize, _a1: u32) { hit(223);  }
unsafe extern "C" fn s224(_a0: usize) -> S_Vector4 { hit(224); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s225(_a0: usize, _a1: usize) { hit(225);  }
unsafe extern "C" fn s226(_a0: usize) -> f32 { hit(226); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s227(_a0: usize, _a1: f32) { hit(227);  }
unsafe extern "C" fn s228(_a0: usize) -> f32 { hit(228); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s229(_a0: usize, _a1: f32) { hit(229);  }
unsafe extern "C" fn s230(_a0: usize) -> usize { hit(230); crate::trace::object(230) }
unsafe extern "C" fn s231(_a0: usize, _a1: usize) { hit(231);  }
unsafe extern "C" fn s232(_a0: usize) -> f32 { hit(232); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s233(_a0: usize, _a1: f32) { hit(233);  }
unsafe extern "C" fn s234(_a0: usize) -> u32 { hit(234); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s235(_a0: usize, _a1: u32) { hit(235);  }
unsafe extern "C" fn s236(_a0: usize) -> usize { hit(236); crate::trace::object(236) }
unsafe extern "C" fn s237(_a0: usize, _a1: usize) { hit(237);  }
unsafe extern "C" fn s238(_a0: usize) -> f32 { hit(238); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s239(_a0: usize, _a1: f32) { hit(239);  }
unsafe extern "C" fn s240(_a0: usize) -> i32 { hit(240); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s241(_a0: usize, _a1: i32) { hit(241);  }
unsafe extern "C" fn s242(_a0: usize) -> f32 { hit(242); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s243(_a0: usize, _a1: f32) { hit(243);  }
unsafe extern "C" fn s244(_a0: usize) -> f32 { hit(244); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s245(_a0: usize, _a1: f32) { hit(245);  }
unsafe extern "C" fn s246() -> usize { hit(246); crate::trace::object(246) }
unsafe extern "C" fn s247(_a0: usize) { hit(247);  }
unsafe extern "C" fn s248(_a0: usize, _a1: f32, _a2: f32, _a3: f32, _a4: f32, _a5: f32, _a6: f32, _a7: f32, _a8: f32, _a9: usize, _a10: usize) { hit(248);  }
unsafe extern "C" fn s249(_a0: usize) -> usize { hit(249); _a0 }
unsafe extern "C" fn s250(_a0: usize) -> usize { hit(250); _a0 }
unsafe extern "C" fn s251(_a0: usize) -> i32 { hit(251); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s252(_a0: usize, _a1: i64, _a2: i32) { hit(252);  }
unsafe extern "C" fn s253(_a0: usize) { hit(253);  }
unsafe extern "C" fn s254(_a0: usize, _a1: usize) { hit(254);  }
unsafe extern "C" fn s255(_a0: usize, _a1: usize, _a2: i32) { hit(255);  }
unsafe extern "C" fn s256(_a0: usize) { hit(256);  }
unsafe extern "C" fn s257(_a0: usize, _a1: i64, _a2: i64) { hit(257);  }
unsafe extern "C" fn s258(_a0: usize, _a1: i64) { hit(258);  }
unsafe extern "C" fn s259(_a0: usize, _a1: i64) -> i32 { hit(259); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s260(_a0: usize) -> i64 { hit(260); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s261(_a0: usize) -> i64 { hit(261); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s262(_a0: usize, _a1: i64) { hit(262);  }
unsafe extern "C" fn s263(_a0: usize, _a1: f32) { hit(263);  }
unsafe extern "C" fn s264(_a0: usize) { hit(264);  }
unsafe extern "C" fn s265(_a0: usize, _a1: usize, _a2: i32) { hit(265);  }
unsafe extern "C" fn s266(_a0: usize) -> S_Vector3 { hit(266); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s267(_a0: usize) -> i32 { hit(267); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s268(_a0: usize, _a1: usize) { hit(268);  }
unsafe extern "C" fn s269(_a0: usize) -> S_Vector4 { hit(269); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s270(_a0: usize, _a1: f32) { hit(270);  }
unsafe extern "C" fn s271(_a0: usize) -> f32 { hit(271); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s272(_a0: usize, _a1: usize) { hit(272);  }
unsafe extern "C" fn s273(_a0: usize) { hit(273);  }
unsafe extern "C" fn s274(_a0: usize, _a1: usize, _a2: S_Sandbox_StringToken, _a3: i32) { hit(274);  }
unsafe extern "C" fn s275(_a0: usize) -> i32 { hit(275); 1 }
unsafe extern "C" fn s276(_a0: usize) -> i32 { hit(276); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s277(_a0: usize) { hit(277);  }
unsafe extern "C" fn s278(_a0: usize) { hit(278);  }
unsafe extern "C" fn s279(_a0: usize) { hit(279);  }
unsafe extern "C" fn s280(_a0: usize) { hit(280);  }
unsafe extern "C" fn s281(_a0: usize, _a1: i32) { hit(281);  }
unsafe extern "C" fn s282(_a0: usize) -> f32 { hit(282); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s283(_a0: usize, _a1: usize) { hit(283);  }
unsafe extern "C" fn s284(_a0: usize) -> S_Transform { hit(284); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s285(_a0: usize, _a1: usize) { hit(285);  }
unsafe extern "C" fn s286(_a0: usize) -> S_BBox { hit(286); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s287(_a0: usize) { hit(287);  }
unsafe extern "C" fn s288(_a0: usize) -> i32 { hit(288); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s289(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize, _a3: u32) { hit(289);  }
unsafe extern "C" fn s290(_a0: usize, _a1: usize) { hit(290);  }
unsafe extern "C" fn s291(_a0: usize) -> usize { hit(291); crate::trace::object(291) }
unsafe extern "C" fn s292(_a0: usize, _a1: u64) { hit(292);  }
unsafe extern "C" fn s293(_a0: usize, _a1: u64) { hit(293);  }
unsafe extern "C" fn s294(_a0: usize, _a1: u64) { hit(294);  }
unsafe extern "C" fn s295(_a0: usize) -> u64 { hit(295); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s296(_a0: usize) -> i32 { hit(296); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s297(_a0: usize, _a1: i32) { hit(297);  }
unsafe extern "C" fn s298(_a0: usize) { hit(298);  }
unsafe extern "C" fn s299(_a0: usize) -> u64 { hit(299); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s300(_a0: usize) -> i32 { hit(300); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s301(_a0: usize) -> usize { hit(301); crate::trace::object(301) }
unsafe extern "C" fn s302(_a0: usize, _a1: S_Sandbox_StringToken) { hit(302);  }
unsafe extern "C" fn s303(_a0: usize, _a1: S_Sandbox_StringToken, _a2: i32) { hit(303);  }
unsafe extern "C" fn s304(_a0: usize, _a1: i32) { hit(304);  }
unsafe extern "C" fn s305(_a0: usize) -> i32 { hit(305); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s306(_a0: usize, _a1: i32) { hit(306);  }
unsafe extern "C" fn s307(_a0: usize, _a1: u32) { hit(307);  }
unsafe extern "C" fn s308(_a0: usize) { hit(308);  }
unsafe extern "C" fn s309(_a0: usize) -> i32 { hit(309); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s310(_a0: usize, _a1: i32) -> u32 { hit(310); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s311(_a0: usize, _a1: u32) { hit(311);  }
unsafe extern "C" fn s312(_a0: usize, _a1: u32) -> i32 { hit(312); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s313(_a0: usize, _a1: S_Sandbox_StringToken) { hit(313);  }
unsafe extern "C" fn s314(_a0: usize, _a1: S_Sandbox_StringToken) { hit(314);  }
unsafe extern "C" fn s315(_a0: usize) -> u32 { hit(315); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s316(_a0: usize, _a1: S_Sandbox_StringToken, _a2: i32) -> i32 { hit(316); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s317(_a0: usize) { hit(317);  }
unsafe extern "C" fn s318(_a0: usize, _a1: i32, _a2: usize) { hit(318);  }
unsafe extern "C" fn s319(_a0: usize) -> usize { hit(319); crate::trace::object(319) }
unsafe extern "C" fn s320(_a0: usize, _a1: usize) { hit(320);  }
unsafe extern "C" fn s321(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize) -> usize { hit(321); crate::trace::object(321) }
unsafe extern "C" fn s322(_a0: usize) -> i32 { hit(322); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s323(_a0: usize, _a1: i32) -> u32 { hit(323); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s324(_a0: usize, _a1: u32, _a2: usize) -> usize { hit(324); crate::trace::object(324) }
unsafe extern "C" fn s325(_a0: usize) -> usize { hit(325); _a0 }
unsafe extern "C" fn s326(_a0: usize) -> usize { hit(326); _a0 }
unsafe extern "C" fn s327(_a0: usize) -> usize { hit(327); _a0 }
unsafe extern "C" fn s328(_a0: usize) -> usize { hit(328); _a0 }
unsafe extern "C" fn s329(_a0: usize) { hit(329);  }
unsafe extern "C" fn s330(_a0: usize, _a1: usize) { hit(330);  }
unsafe extern "C" fn s331(_a0: usize) { hit(331);  }
unsafe extern "C" fn s332(_a0: usize) { hit(332);  }
unsafe extern "C" fn s333(_a0: usize, _a1: usize) { hit(333);  }
unsafe extern "C" fn s334(_a0: usize) -> S_Vector3 { hit(334); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s335(_a0: usize, _a1: usize) { hit(335);  }
unsafe extern "C" fn s336(_a0: usize) -> S_Vector3 { hit(336); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s337(_a0: usize, _a1: usize) { hit(337);  }
unsafe extern "C" fn s338(_a0: usize, _a1: usize) { hit(338);  }
unsafe extern "C" fn s339(_a0: usize) -> S_Vector3 { hit(339); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s340(_a0: usize, _a1: f32) { hit(340);  }
unsafe extern "C" fn s341(_a0: usize) -> f32 { hit(341); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s342(_a0: usize, _a1: f32) { hit(342);  }
unsafe extern "C" fn s343(_a0: usize) -> f32 { hit(343); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s344(_a0: usize, _a1: f32) { hit(344);  }
unsafe extern "C" fn s345(_a0: usize) -> f32 { hit(345); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s346(_a0: usize, _a1: f32) { hit(346);  }
unsafe extern "C" fn s347(_a0: usize) -> f32 { hit(347); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s348(_a0: usize) -> i32 { hit(348); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s349(_a0: usize, _a1: i32) { hit(349);  }
unsafe extern "C" fn s350(_a0: usize) -> i32 { hit(350); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s351(_a0: usize, _a1: i32) { hit(351);  }
unsafe extern "C" fn s352(_a0: usize, _a1: f32) { hit(352);  }
unsafe extern "C" fn s353(_a0: usize) -> f32 { hit(353); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s354(_a0: usize, _a1: f32) { hit(354);  }
unsafe extern "C" fn s355(_a0: usize) -> f32 { hit(355); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s356(_a0: usize, _a1: f32) { hit(356);  }
unsafe extern "C" fn s357(_a0: usize) -> f32 { hit(357); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s358(_a0: usize, _a1: usize) { hit(358);  }
unsafe extern "C" fn s359(_a0: usize) -> usize { hit(359); crate::trace::object(359) }
unsafe extern "C" fn s360(_a0: usize, _a1: u32, _a2: u32) { hit(360);  }
unsafe extern "C" fn s361(_a0: usize) -> i32 { hit(361); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s362(_a0: usize, _a1: i32) { hit(362);  }
unsafe extern "C" fn s363(_a0: usize) -> f32 { hit(363); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s364(_a0: usize, _a1: f32) { hit(364);  }
unsafe extern "C" fn s365(_a0: usize) -> f32 { hit(365); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s366(_a0: usize, _a1: f32) { hit(366);  }
unsafe extern "C" fn s367(_a0: usize) -> f32 { hit(367); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s368(_a0: usize, _a1: f32) { hit(368);  }
unsafe extern "C" fn s369(_a0: usize) -> i32 { hit(369); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s370(_a0: usize, _a1: i32) { hit(370);  }
unsafe extern "C" fn s371(_a0: usize, _a1: i32) { hit(371);  }
unsafe extern "C" fn s372(_a0: usize, _a1: f32) { hit(372);  }
unsafe extern "C" fn s373(_a0: usize, _a1: i32) { hit(373);  }
unsafe extern "C" fn s374(_a0: usize, _a1: i32) { hit(374);  }
unsafe extern "C" fn s375(_a0: usize, _a1: i32) { hit(375);  }
unsafe extern "C" fn s376(_a0: usize, _a1: i32) { hit(376);  }
unsafe extern "C" fn s377(_a0: usize, _a1: f32) { hit(377);  }
unsafe extern "C" fn s378(_a0: usize, _a1: f32) { hit(378);  }
unsafe extern "C" fn s379(_a0: usize, _a1: i32) { hit(379);  }
unsafe extern "C" fn s380(_a0: usize, _a1: i32) { hit(380);  }
unsafe extern "C" fn s381(_a0: usize) -> i32 { hit(381); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s382(_a0: usize) -> i32 { hit(382); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s383(_a0: usize) -> u32 { hit(383); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s384(_a0: usize, _a1: u32) { hit(384);  }
unsafe extern "C" fn s385(_a0: usize) -> i64 { hit(385); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s386(_a0: usize, _a1: i64) { hit(386);  }
unsafe extern "C" fn s387(_a0: usize, _a1: f32) { hit(387);  }
unsafe extern "C" fn s388(_a0: usize, _a1: f32) { hit(388);  }
unsafe extern "C" fn s389(_a0: usize) -> i32 { hit(389); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s390(_a0: usize) -> i64 { hit(390); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s391(_a0: usize, _a1: i64) { hit(391);  }
unsafe extern "C" fn s392(_a0: usize) -> S_Vector3 { hit(392); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s393(_a0: usize, _a1: usize) { hit(393);  }
unsafe extern "C" fn s394(_a0: usize) -> S_Vector3 { hit(394); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s395(_a0: usize, _a1: usize) { hit(395);  }
unsafe extern "C" fn s396(_a0: usize) -> S_Vector3 { hit(396); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s397(_a0: usize, _a1: usize) { hit(397);  }
unsafe extern "C" fn s398(_a0: usize) -> i32 { hit(398); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s399(_a0: usize, _a1: i32) { hit(399);  }
unsafe extern "C" fn s400(_a0: usize) -> usize { hit(400); crate::trace::object(400) }
unsafe extern "C" fn s401(_a0: usize, _a1: usize) { hit(401);  }
unsafe extern "C" fn s402(_a0: usize) -> S_Vector4 { hit(402); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s403(_a0: usize, _a1: usize) { hit(403);  }
unsafe extern "C" fn s404(_a0: usize) -> f32 { hit(404); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s405(_a0: usize, _a1: f32) { hit(405);  }
unsafe extern "C" fn s406() -> usize { hit(406); crate::trace::object(406) }
unsafe extern "C" fn s407(_a0: usize) { hit(407);  }
unsafe extern "C" fn s408(_a0: usize, _a1: usize, _a2: usize, _a3: f32, _a4: f32, _a5: f32, _a6: f32, _a7: f32, _a8: f32, _a9: f32, _a10: f32) { hit(408);  }
unsafe extern "C" fn s409(_a0: usize, _a1: usize, _a2: usize, _a3: f32, _a4: f32, _a5: f32, _a6: f32) { hit(409);  }
unsafe extern "C" fn s410(_a0: usize, _a1: usize, _a2: usize, _a3: f32, _a4: f32, _a5: f32, _a6: f32) { hit(410);  }
unsafe extern "C" fn s411(_a0: usize, _a1: f32, _a2: f32) { hit(411);  }
unsafe extern "C" fn s412(_a0: usize, _a1: usize, _a2: f32, _a3: f32, _a4: f32, _a5: f32, _a6: usize, _a7: usize, _a8: usize) { hit(412);  }
unsafe extern "C" fn s413(_a0: usize) -> S_Matrix { hit(413); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s414(_a0: usize) -> S_Matrix { hit(414); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s415(_a0: usize) -> S_Matrix { hit(415); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s416(_a0: usize) -> S_Matrix { hit(416); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s417(_a0: usize) -> S_Matrix { hit(417); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s418(_a0: usize) -> S_Matrix { hit(418); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s419(_a0: usize) -> S_Matrix { hit(419); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s420(_a0: usize) -> S_Matrix { hit(420); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s421(_a0: usize) -> S_Matrix { hit(421); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s422(_a0: usize) -> S_Matrix { hit(422); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s423(_a0: usize) -> S_Matrix { hit(423); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s424(_a0: usize, _a1: usize) -> i32 { hit(424); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s425(_a0: usize, _a1: usize, _a2: usize) -> i32 { hit(425); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s426(_a0: usize, _a1: usize, _a2: usize) { hit(426);  }
unsafe extern "C" fn s427(_a0: usize, _a1: usize, _a2: usize) { hit(427);  }
unsafe extern "C" fn s428(_a0: usize) -> i32 { hit(428); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s429(_a0: usize) -> S_Vector3 { hit(429); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s430(_a0: usize) -> S_Angles { hit(430); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s431(_a0: usize) -> f32 { hit(431); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s432(_a0: usize) -> f32 { hit(432); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s433(_a0: usize) -> f32 { hit(433); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s434(_a0: usize) -> f32 { hit(434); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s435(_a0: usize, _a1: i32, _a2: usize, _a3: usize) { hit(435);  }
unsafe extern "C" fn s436(_a0: usize, _a1: usize, _a2: f32) -> f32 { hit(436); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s437(_a0: usize, _a1: i32) -> usize { hit(437); crate::trace::object(437) }
unsafe extern "C" fn s438(_a0: usize) -> S_Vector3 { hit(438); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s439(_a0: usize, _a1: usize) { hit(439);  }
unsafe extern "C" fn s440(_a0: usize) -> S_Vector3 { hit(440); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s441(_a0: usize, _a1: usize) { hit(441);  }
unsafe extern "C" fn s442(_a0: usize) -> usize { hit(442); crate::trace::object(442) }
unsafe extern "C" fn s443(_a0: usize, _a1: usize) { hit(443);  }
unsafe extern "C" fn s444(_a0: usize) -> usize { hit(444); crate::trace::object(444) }
unsafe extern "C" fn s445(_a0: usize, _a1: usize) { hit(445);  }
unsafe extern "C" fn s446(_a0: usize) -> usize { hit(446); crate::trace::object(446) }
unsafe extern "C" fn s447(_a0: usize, _a1: usize) { hit(447);  }
unsafe extern "C" fn s448(_a0: usize) -> f32 { hit(448); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s449(_a0: usize, _a1: f32) { hit(449);  }
unsafe extern "C" fn s450(_a0: usize) -> u32 { hit(450); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s451(_a0: usize, _a1: u32) { hit(451);  }
unsafe extern "C" fn s452(_a0: usize) -> S_Color32 { hit(452); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s453(_a0: usize, _a1: S_Color32) { hit(453);  }
unsafe extern "C" fn s454(_a0: usize) -> u16 { hit(454); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s455(_a0: usize, _a1: u16) { hit(455);  }
unsafe extern "C" fn s456(_a0: usize) -> u8 { hit(456); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s457(_a0: usize, _a1: u8) { hit(457);  }
unsafe extern "C" fn s458(_a0: usize) -> i32 { hit(458); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s459(_a0: usize, _a1: i32) { hit(459);  }
unsafe extern "C" fn s460(_a0: usize) -> i32 { hit(460); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s461(_a0: usize, _a1: i32) { hit(461);  }
unsafe extern "C" fn s462(_a0: usize) -> i32 { hit(462); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s463(_a0: usize, _a1: i32) { hit(463);  }
unsafe extern "C" fn s464(_a0: usize) -> i32 { hit(464); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s465(_a0: usize, _a1: i32) { hit(465);  }
unsafe extern "C" fn s466(_a0: usize) -> i32 { hit(466); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s467(_a0: usize, _a1: i32) -> usize { hit(467); crate::trace::object(467) }
unsafe extern "C" fn s468(_a0: usize) -> usize { hit(468); crate::trace::object(468) }
unsafe extern "C" fn s469(_a0: usize, _a1: usize) { hit(469);  }
unsafe extern "C" fn s470(_a0: usize) -> usize { hit(470); crate::trace::object(470) }
unsafe extern "C" fn s471(_a0: usize, _a1: usize) { hit(471);  }
unsafe extern "C" fn s472(_a0: usize) -> usize { hit(472); _a0 }
unsafe extern "C" fn s473(_a0: usize) -> usize { hit(473); _a0 }
unsafe extern "C" fn s474(_a0: usize) -> i32 { hit(474); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s475(_a0: usize, _a1: i64, _a2: i64) { hit(475);  }
unsafe extern "C" fn s476(_a0: usize, _a1: i64) { hit(476);  }
unsafe extern "C" fn s477(_a0: usize, _a1: i64) -> i32 { hit(477); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s478(_a0: usize) -> i64 { hit(478); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s479(_a0: usize) -> i64 { hit(479); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s480(_a0: usize, _a1: i64) { hit(480);  }
unsafe extern "C" fn s481(_a0: usize, _a1: f32) { hit(481);  }
unsafe extern "C" fn s482(_a0: usize) { hit(482);  }
unsafe extern "C" fn s483(_a0: usize, _a1: usize, _a2: i32) { hit(483);  }
unsafe extern "C" fn s484(_a0: usize) -> S_Vector3 { hit(484); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s485(_a0: usize) -> i32 { hit(485); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s486(_a0: usize, _a1: usize) { hit(486);  }
unsafe extern "C" fn s487(_a0: usize) -> S_Vector4 { hit(487); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s488(_a0: usize, _a1: f32) { hit(488);  }
unsafe extern "C" fn s489(_a0: usize) -> f32 { hit(489); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s490(_a0: usize, _a1: usize) { hit(490);  }
unsafe extern "C" fn s491(_a0: usize) { hit(491);  }
unsafe extern "C" fn s492(_a0: usize, _a1: usize, _a2: S_Sandbox_StringToken, _a3: i32) { hit(492);  }
unsafe extern "C" fn s493(_a0: usize) -> i32 { hit(493); 1 }
unsafe extern "C" fn s494(_a0: usize) -> i32 { hit(494); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s495(_a0: usize) { hit(495);  }
unsafe extern "C" fn s496(_a0: usize) { hit(496);  }
unsafe extern "C" fn s497(_a0: usize) { hit(497);  }
unsafe extern "C" fn s498(_a0: usize) { hit(498);  }
unsafe extern "C" fn s499(_a0: usize, _a1: i32) { hit(499);  }
unsafe extern "C" fn s500(_a0: usize) -> f32 { hit(500); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s501(_a0: usize, _a1: usize) { hit(501);  }
unsafe extern "C" fn s502(_a0: usize) -> S_Transform { hit(502); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s503(_a0: usize, _a1: usize) { hit(503);  }
unsafe extern "C" fn s504(_a0: usize) -> S_BBox { hit(504); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s505(_a0: usize) { hit(505);  }
unsafe extern "C" fn s506(_a0: usize) -> i32 { hit(506); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s507(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize, _a3: u32) { hit(507);  }
unsafe extern "C" fn s508(_a0: usize, _a1: usize) { hit(508);  }
unsafe extern "C" fn s509(_a0: usize) -> usize { hit(509); crate::trace::object(509) }
unsafe extern "C" fn s510(_a0: usize, _a1: u64) { hit(510);  }
unsafe extern "C" fn s511(_a0: usize, _a1: u64) { hit(511);  }
unsafe extern "C" fn s512(_a0: usize, _a1: u64) { hit(512);  }
unsafe extern "C" fn s513(_a0: usize) -> u64 { hit(513); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s514(_a0: usize) -> i32 { hit(514); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s515(_a0: usize, _a1: i32) { hit(515);  }
unsafe extern "C" fn s516(_a0: usize) { hit(516);  }
unsafe extern "C" fn s517(_a0: usize) -> u64 { hit(517); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s518(_a0: usize) -> i32 { hit(518); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s519(_a0: usize) -> usize { hit(519); crate::trace::object(519) }
unsafe extern "C" fn s520(_a0: usize, _a1: S_Sandbox_StringToken) { hit(520);  }
unsafe extern "C" fn s521(_a0: usize, _a1: S_Sandbox_StringToken, _a2: i32) { hit(521);  }
unsafe extern "C" fn s522(_a0: usize, _a1: i32) { hit(522);  }
unsafe extern "C" fn s523(_a0: usize) -> i32 { hit(523); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s524(_a0: usize, _a1: i32) { hit(524);  }
unsafe extern "C" fn s525(_a0: usize, _a1: u32) { hit(525);  }
unsafe extern "C" fn s526(_a0: usize) { hit(526);  }
unsafe extern "C" fn s527(_a0: usize) -> i32 { hit(527); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s528(_a0: usize, _a1: i32) -> u32 { hit(528); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s529(_a0: usize, _a1: u32) { hit(529);  }
unsafe extern "C" fn s530(_a0: usize, _a1: u32) -> i32 { hit(530); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s531(_a0: usize, _a1: S_Sandbox_StringToken) { hit(531);  }
unsafe extern "C" fn s532(_a0: usize, _a1: S_Sandbox_StringToken) { hit(532);  }
unsafe extern "C" fn s533(_a0: usize) -> u32 { hit(533); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s534(_a0: usize, _a1: S_Sandbox_StringToken, _a2: i32) -> i32 { hit(534); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s535(_a0: usize) { hit(535);  }
unsafe extern "C" fn s536(_a0: usize, _a1: i32, _a2: usize) { hit(536);  }
unsafe extern "C" fn s537(_a0: usize) -> usize { hit(537); crate::trace::object(537) }
unsafe extern "C" fn s538(_a0: usize) { hit(538);  }
unsafe extern "C" fn s539(_a0: usize) -> i32 { hit(539); unsafe { crate::impls::CMtrlSystm2ppSys_Init(_a0) } }
unsafe extern "C" fn s540(_a0: usize) -> i32 { hit(540); unsafe { crate::impls::CMtrlSystm2ppSys_InitWithoutMaterialSystem(_a0) } }
unsafe extern "C" fn s541(_a0: usize) -> i32 { hit(541); unsafe { crate::impls::CMtrlSystm2ppSys_InitFinishSetupMaterialSystem(_a0) } }
unsafe extern "C" fn s542(_a0: usize) { hit(542);  }
unsafe extern "C" fn s543(_a0: usize, _a1: i32) { hit(543);  }
unsafe extern "C" fn s544(_a0: usize, _a1: usize) { hit(544);  }
unsafe extern "C" fn s545(_a0: usize, _a1: usize) { hit(545);  }
unsafe extern "C" fn s546(_a0: usize, _a1: usize, _a2: i32) { hit(546);  }
unsafe extern "C" fn s547(_a0: usize) { hit(547);  }
unsafe extern "C" fn s548(_a0: usize, _a1: usize) { hit(548);  }
unsafe extern "C" fn s549(_a0: usize, _a1: i32) { hit(549);  }
unsafe extern "C" fn s550(_a0: usize) -> i32 { hit(550); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s551(_a0: usize) { hit(551);  }
unsafe extern "C" fn s552(_a0: usize, _a1: u32) -> i32 { hit(552); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s553(_a0: usize) { hit(553);  }
unsafe extern "C" fn s554(_a0: usize) -> i32 { hit(554); unsafe { crate::impls::CMtrlSystm2ppSys_IsConsoleApp(_a0) } }
unsafe extern "C" fn s555(_a0: usize) -> i32 { hit(555); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s556(_a0: usize, _a1: i32) { hit(556); unsafe { crate::impls::CMtrlSystm2ppSys_SetDedicatedServer(_a0, _a1) } }
unsafe extern "C" fn s557(_a0: usize) -> i32 { hit(557); unsafe { crate::impls::CMtrlSystm2ppSys_IsDedicatedServer(_a0) } }
unsafe extern "C" fn s558(_a0: usize) -> usize { hit(558); unsafe { crate::impls::CMtrlSystm2ppSys_GetContentPath(_a0) } }
unsafe extern "C" fn s559(_a0: usize) -> usize { hit(559); crate::trace::object(559) }
unsafe extern "C" fn s560(_a0: usize) { hit(560); unsafe { crate::impls::CMtrlSystm2ppSys_SetInToolsMode(_a0) } }
unsafe extern "C" fn s561(_a0: usize) -> i32 { hit(561); unsafe { crate::impls::CMtrlSystm2ppSys_IsInToolsMode(_a0) } }
unsafe extern "C" fn s562(_a0: usize) -> i32 { hit(562); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s563(_a0: usize) -> i32 { hit(563); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s564(_a0: usize) { hit(564);  }
unsafe extern "C" fn s565(_a0: usize) -> i32 { hit(565); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s566(_a0: usize, _a1: usize, _a2: usize) -> usize { hit(566); crate::trace::object(566) }
unsafe extern "C" fn s567(_a0: usize) { hit(567); unsafe { crate::impls::CMtrlSystm2ppSys_SetInTestMode(_a0) } }
unsafe extern "C" fn s568(_a0: usize) -> i32 { hit(568); unsafe { crate::impls::CMtrlSystm2ppSys_IsInTestMode(_a0) } }
unsafe extern "C" fn s569(_a0: usize) { hit(569); unsafe { crate::impls::CMtrlSystm2ppSys_SetInStandaloneApp(_a0) } }
unsafe extern "C" fn s570(_a0: usize) -> i32 { hit(570); unsafe { crate::impls::CMtrlSystm2ppSys_IsStandaloneApp(_a0) } }
unsafe extern "C" fn s571(_a0: usize, _a1: u32) { hit(571); unsafe { crate::impls::CMtrlSystm2ppSys_SetSteamAppId(_a0, _a1) } }
unsafe extern "C" fn s572(_a0: usize) -> u32 { hit(572); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s573(_a0: usize) { hit(573);  }
unsafe extern "C" fn s574(_a0: usize) -> i32 { hit(574); 1 }
unsafe extern "C" fn s575(_a0: usize) -> i32 { hit(575); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s576(_a0: usize) -> i32 { hit(576); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s577(_a0: usize) -> usize { hit(577); crate::trace::object(577) }
unsafe extern "C" fn s578(_a0: usize) -> usize { hit(578); crate::trace::object(578) }
unsafe extern "C" fn s579(_a0: usize) -> S_System_Guid { hit(579); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s580(_a0: usize) -> usize { hit(580); crate::trace::object(580) }
unsafe extern "C" fn s581(_a0: usize) -> usize { hit(581); crate::trace::object(581) }
unsafe extern "C" fn s582(_a0: usize) -> usize { hit(582); crate::trace::object(582) }
unsafe extern "C" fn s583(_a0: usize) -> i32 { hit(583); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s584(_a0: usize) -> i32 { hit(584); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s585(_a0: usize) -> i32 { hit(585); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s586(_a0: usize, _a1: usize) -> usize { hit(586); crate::trace::object(586) }
unsafe extern "C" fn s587(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize) -> i32 { hit(587); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s588(_a0: usize, _a1: i32) -> usize { hit(588); crate::trace::object(588) }
unsafe extern "C" fn s589(_a0: usize, _a1: usize) -> i32 { hit(589); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s590(_a0: usize, _a1: i32) -> usize { hit(590); crate::trace::object(590) }
unsafe extern "C" fn s591(_a0: usize) -> i32 { hit(591); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s592(_a0: usize, _a1: i32) -> i32 { hit(592); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s593(_a0: usize, _a1: i32) -> u64 { hit(593); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s594(_a0: usize, _a1: i32, _a2: i32) -> u64 { hit(594); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s595(_a0: usize, _a1: i32, _a2: u64) -> i32 { hit(595); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s596(_a0: usize) -> i32 { hit(596); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s597(_a0: usize) -> u64 { hit(597); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s598(_a0: usize, _a1: i32) -> u64 { hit(598); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s599(_a0: usize, _a1: i32, _a2: i32) -> usize { hit(599); crate::trace::object(599) }
unsafe extern "C" fn s600(_a0: usize) -> i32 { hit(600); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s601(_a0: usize, _a1: i32) -> usize { hit(601); crate::trace::object(601) }
unsafe extern "C" fn s602(_a0: usize, _a1: S_Sandbox_StringToken) -> i32 { hit(602); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s603(_a0: usize, _a1: i32) -> i32 { hit(603); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s604(_a0: usize, _a1: i32, _a2: i32) -> usize { hit(604); crate::trace::object(604) }
unsafe extern "C" fn s605(_a0: usize, _a1: i32) -> i32 { hit(605); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s606(_a0: usize, _a1: i32, _a2: i32) -> i32 { hit(606); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s607(_a0: usize, _a1: i32, _a2: i32, _a3: i32) -> usize { hit(607); crate::trace::object(607) }
unsafe extern "C" fn s608(_a0: usize, _a1: usize, _a2: usize, _a3: usize) { hit(608);  }
unsafe extern "C" fn s609(_a0: usize, _a1: i32, _a2: usize, _a3: usize, _a4: usize, _a5: usize, _a6: usize, _a7: usize, _a8: usize) { hit(609);  }
unsafe extern "C" fn s610(_a0: usize, _a1: i32) -> usize { hit(610); crate::trace::object(610) }
unsafe extern "C" fn s611(_a0: usize, _a1: i32, _a2: i32, _a3: usize, _a4: usize, _a5: usize, _a6: usize, _a7: usize, _a8: usize, _a9: usize, _a10: usize) { hit(611);  }
unsafe extern "C" fn s612(_a0: usize, _a1: i32, _a2: i32) -> usize { hit(612); crate::trace::object(612) }
unsafe extern "C" fn s613(_a0: usize) -> i32 { hit(613); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s614(_a0: usize) -> i32 { hit(614); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s615(_a0: usize, _a1: f32, _a2: f32) -> i32 { hit(615); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s616(_a0: usize) -> i32 { hit(616); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s617(_a0: usize, _a1: i32) -> f32 { hit(617); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s618(_a0: usize) -> S_BBox { hit(618); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s619(_a0: usize) -> S_BBox { hit(619); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s620(_a0: usize) -> S_BBox { hit(620); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s621(_a0: usize) -> i32 { hit(621); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s622(_a0: usize, _a1: usize) -> i32 { hit(622); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s623(_a0: usize, _a1: i32) -> usize { hit(623); crate::trace::object(623) }
unsafe extern "C" fn s624(_a0: usize, _a1: i32) -> i32 { hit(624); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s625(_a0: usize, _a1: i32) -> S_Transform { hit(625); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s626(_a0: usize, _a1: i32) -> S_Vector3 { hit(626); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s627(_a0: usize, _a1: i32) -> S_Rotation { hit(627); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s628(_a0: usize) -> i32 { hit(628); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s629(_a0: usize, _a1: i32) -> usize { hit(629); crate::trace::object(629) }
unsafe extern "C" fn s630(_a0: usize, _a1: usize) -> i32 { hit(630); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s631(_a0: usize, _a1: i32, _a2: i32) -> f32 { hit(631); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s632(_a0: usize) -> i32 { hit(632); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s633(_a0: usize, _a1: i32) -> usize { hit(633); crate::trace::object(633) }
unsafe extern "C" fn s634(_a0: usize, _a1: usize) { hit(634);  }
unsafe extern "C" fn s635(_a0: usize) -> i32 { hit(635); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s636(_a0: usize, _a1: usize, _a2: usize) -> i32 { hit(636); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s637(_a0: usize) -> usize { hit(637); crate::trace::object(637) }
unsafe extern "C" fn s638(_a0: usize) -> usize { hit(638); crate::trace::object(638) }
unsafe extern "C" fn s639(_a0: usize, _a1: usize, _a2: i32) -> usize { hit(639); crate::trace::object(639) }
unsafe extern "C" fn s640(_a0: usize, _a1: usize, _a2: i32) -> i32 { hit(640); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s641(_a0: usize, _a1: i32, _a2: i32) -> usize { hit(641); crate::trace::object(641) }
unsafe extern "C" fn s642(_a0: usize, _a1: usize) -> i32 { hit(642); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s643(_a0: usize, _a1: i32) -> i32 { hit(643); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s644(_a0: usize) -> i32 { hit(644); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s645(_a0: usize) -> i32 { hit(645); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s646(_a0: usize, _a1: i32) -> i64 { hit(646); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s647(_a0: usize) -> i32 { hit(647); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s648(_a0: usize, _a1: i32) -> usize { hit(648); crate::trace::object(648) }
unsafe extern "C" fn s649(_a0: usize) -> i32 { hit(649); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s650(_a0: usize, _a1: i32) -> usize { hit(650); crate::trace::object(650) }
unsafe extern "C" fn s651(_a0: usize) { hit(651);  }
unsafe extern "C" fn s652() -> usize { hit(652); crate::trace::object(652) }
unsafe extern "C" fn s653(_a0: usize, _a1: f32) { hit(653);  }
unsafe extern "C" fn s654(_a0: usize, _a1: usize, _a2: usize) { hit(654);  }
unsafe extern "C" fn s655(_a0: usize, _a1: usize, _a2: usize) { hit(655);  }
unsafe extern "C" fn s656(_a0: usize, _a1: usize) { hit(656);  }
unsafe extern "C" fn s657(_a0: usize, _a1: f32) { hit(657);  }
unsafe extern "C" fn s658(_a0: usize, _a1: usize) -> i32 { hit(658); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s659(_a0: usize, _a1: i32, _a2: f32) { hit(659);  }
unsafe extern "C" fn s660(_a0: usize, _a1: usize, _a2: u64, _a3: i32) { hit(660);  }
unsafe extern "C" fn s661(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize) { hit(661);  }
unsafe extern "C" fn s662(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize) { hit(662);  }
unsafe extern "C" fn s663(_a0: usize, _a1: usize) -> i32 { hit(663); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s664(_a0: usize) -> usize { hit(664); crate::trace::object(664) }
unsafe extern "C" fn s665(_a0: usize, _a1: usize, _a2: usize) -> usize { hit(665); crate::trace::object(665) }
unsafe extern "C" fn s666(_a0: usize, _a1: u64) { hit(666);  }
unsafe extern "C" fn s667(_a0: usize, _a1: usize) { hit(667);  }
unsafe extern "C" fn s668(_a0: usize, _a1: i32) { hit(668);  }
unsafe extern "C" fn s669(_a0: usize, _a1: usize) { hit(669);  }
unsafe extern "C" fn s670(_a0: usize, _a1: usize, _a2: i32, _a3: usize, _a4: i32) { hit(670);  }
unsafe extern "C" fn s671(_a0: usize, _a1: usize, _a2: i32) -> usize { hit(671); crate::trace::object(671) }
unsafe extern "C" fn s672() -> usize { hit(672); crate::trace::object(672) }
unsafe extern "C" fn s673(_a0: usize) { hit(673);  }
unsafe extern "C" fn s674(_a0: usize, _a1: f32, _a2: f32, _a3: f32, _a4: f32, _a5: f32, _a6: f32, _a7: f32, _a8: usize, _a9: usize) { hit(674);  }
unsafe extern "C" fn s675(_a0: usize, _a1: usize) { hit(675);  }
unsafe extern "C" fn s676(_a0: usize, _a1: S_Sandbox_StringToken) { hit(676);  }
unsafe extern "C" fn s677(_a0: usize, _a1: usize) { hit(677);  }
unsafe extern "C" fn s678(_a0: usize, _a1: usize) { hit(678);  }
unsafe extern "C" fn s679(_a0: usize, _a1: usize) { hit(679);  }
unsafe extern "C" fn s680(_a0: usize, _a1: usize, _a2: usize) { hit(680);  }
unsafe extern "C" fn s681(_a0: usize, _a1: usize, _a2: i32, _a3: usize) { hit(681);  }
unsafe extern "C" fn s682(_a0: usize, _a1: usize, _a2: i32, _a3: usize, _a4: f32, _a5: f32, _a6: i32, _a7: i32, _a8: i32, _a9: i32) { hit(682);  }
unsafe extern "C" fn s683(_a0: usize, _a1: usize, _a2: u32, _a3: usize, _a4: u32, _a5: usize) { hit(683);  }
unsafe extern "C" fn s684(_a0: usize) -> f32 { hit(684); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s685(_a0: usize, _a1: f32) { hit(685);  }
unsafe extern "C" fn s686(_a0: usize) -> f32 { hit(686); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s687(_a0: usize, _a1: f32) { hit(687);  }
unsafe extern "C" fn s688(_a0: usize) -> f32 { hit(688); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s689(_a0: usize, _a1: f32) { hit(689);  }
unsafe extern "C" fn s690(_a0: usize) -> f32 { hit(690); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s691(_a0: usize, _a1: f32) { hit(691);  }
unsafe extern "C" fn s692(_a0: usize) { hit(692);  }
unsafe extern "C" fn s693(_a0: i32, _a1: i32) -> usize { hit(693); crate::trace::object(693) }
unsafe extern "C" fn s694(_a0: usize, _a1: i32) -> usize { hit(694); crate::trace::object(694) }
unsafe extern "C" fn s695(_a0: usize, _a1: i32) -> usize { hit(695); crate::trace::object(695) }
unsafe extern "C" fn s696(_a0: usize, _a1: i32) { hit(696);  }
unsafe extern "C" fn s697(_a0: usize, _a1: f32, _a2: f32, _a3: f32, _a4: f32, _a5: f32) { hit(697);  }
unsafe extern "C" fn s698(_a0: usize) -> usize { hit(698); crate::trace::object(698) }
unsafe extern "C" fn s699(_a0: usize, _a1: usize) { hit(699);  }
unsafe extern "C" fn s700(_a0: usize) -> u32 { hit(700); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s701(_a0: usize, _a1: u32) { hit(701);  }
unsafe extern "C" fn s702(_a0: usize) -> u32 { hit(702); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s703(_a0: usize, _a1: u32) { hit(703);  }
unsafe extern "C" fn s704(_a0: usize) -> i32 { hit(704); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s705(_a0: usize, _a1: i32) { hit(705);  }
unsafe extern "C" fn s706(_a0: usize) -> i32 { hit(706); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s707(_a0: usize, _a1: i32) { hit(707);  }
unsafe extern "C" fn s708(_a0: usize) -> i32 { hit(708); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s709(_a0: usize, _a1: i32) { hit(709);  }
unsafe extern "C" fn s710(_a0: usize) -> i32 { hit(710); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s711(_a0: usize, _a1: i32) { hit(711);  }
unsafe extern "C" fn s712(_a0: usize) -> i32 { hit(712); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s713(_a0: usize, _a1: i32) { hit(713);  }
unsafe extern "C" fn s714(_a0: usize) -> usize { hit(714); crate::trace::object(714) }
unsafe extern "C" fn s715(_a0: usize, _a1: usize) { hit(715);  }
unsafe extern "C" fn s716(_a0: usize) { hit(716);  }
unsafe extern "C" fn s717(_a0: usize) -> i32 { hit(717); 1 }
unsafe extern "C" fn s718(_a0: usize) -> i32 { hit(718); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s719(_a0: usize) -> i32 { hit(719); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s720(_a0: usize) -> usize { hit(720); crate::trace::object(720) }
unsafe extern "C" fn s721(_a0: usize) -> usize { hit(721); crate::trace::object(721) }
unsafe extern "C" fn s722(_a0: usize) -> S_System_Guid { hit(722); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s723(_a0: usize) -> usize { hit(723); crate::trace::object(723) }
unsafe extern "C" fn s724(_a0: usize) -> u32 { hit(724); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s725(_a0: usize) -> i32 { hit(725); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s726(_a0: usize, _a1: i32) -> usize { hit(726); crate::trace::object(726) }
unsafe extern "C" fn s727(_a0: usize) -> i32 { hit(727); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s728(_a0: usize, _a1: i32) -> u32 { hit(728); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s729(_a0: usize, _a1: i32) -> S_Transform { hit(729); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s730(_a0: usize, _a1: i32) -> usize { hit(730); crate::trace::object(730) }
unsafe extern "C" fn s731(_a0: usize, _a1: i32) -> usize { hit(731); crate::trace::object(731) }
unsafe extern "C" fn s732(_a0: usize) -> i32 { hit(732); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s733(_a0: usize) -> i32 { hit(733); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s734(_a0: usize, _a1: i32, _a2: i32) -> usize { hit(734); crate::trace::object(734) }
unsafe extern "C" fn s735(_a0: usize, _a1: i32) -> i32 { hit(735); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s736(_a0: usize) -> i32 { hit(736); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s737(_a0: usize, _a1: i32) -> usize { hit(737); crate::trace::object(737) }
unsafe extern "C" fn s738(_a0: usize) -> u16 { hit(738); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s739(_a0: usize, _a1: u16) { hit(739);  }
unsafe extern "C" fn s740(_a0: usize) { hit(740);  }
unsafe extern "C" fn s741() -> usize { hit(741); crate::trace::object(741) }
unsafe extern "C" fn s742(_a0: usize) -> i32 { hit(742); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s743(_a0: usize, _a1: i32) -> i32 { hit(743); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s744(_a0: usize) { hit(744);  }
unsafe extern "C" fn s745(_a0: usize) -> usize { hit(745); _a0 }
unsafe extern "C" fn s746(_a0: usize) -> usize { hit(746); _a0 }
unsafe extern "C" fn s747(_a0: usize) { hit(747);  }
unsafe extern "C" fn s748() -> usize { hit(748); crate::trace::object(748) }
unsafe extern "C" fn s749(_a0: usize, _a1: i32) { hit(749);  }
unsafe extern "C" fn s750(_a0: usize) -> i32 { hit(750); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s751(_a0: usize) { hit(751);  }
unsafe extern "C" fn s752(_a0: usize) -> usize { hit(752); _a0 }
unsafe extern "C" fn s753(_a0: usize) -> usize { hit(753); _a0 }
unsafe extern "C" fn s754(_a0: usize) { hit(754);  }
unsafe extern "C" fn s755() -> usize { hit(755); crate::trace::object(755) }
unsafe extern "C" fn s756(_a0: usize, _a1: i32) { hit(756);  }
unsafe extern "C" fn s757(_a0: usize) -> i32 { hit(757); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s758(_a0: usize) { hit(758);  }
unsafe extern "C" fn s759(_a0: usize) { hit(759);  }
unsafe extern "C" fn s760() -> usize { hit(760); crate::trace::object(760) }
unsafe extern "C" fn s761(_a0: usize, _a1: S_Sandbox_StringToken, _a2: f32) { hit(761);  }
unsafe extern "C" fn s762(_a0: usize, _a1: S_Sandbox_StringToken, _a2: f32) -> f32 { hit(762); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s763(_a0: usize, _a1: S_Sandbox_StringToken) { hit(763);  }
unsafe extern "C" fn s764(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize) { hit(764);  }
unsafe extern "C" fn s765(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize) -> S_Vector2 { hit(765); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s766(_a0: usize, _a1: S_Sandbox_StringToken) { hit(766);  }
unsafe extern "C" fn s767(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize) { hit(767);  }
unsafe extern "C" fn s768(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize) -> S_Vector3 { hit(768); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s769(_a0: usize, _a1: S_Sandbox_StringToken) { hit(769);  }
unsafe extern "C" fn s770(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize) { hit(770);  }
unsafe extern "C" fn s771(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize) -> S_Vector4 { hit(771); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s772(_a0: usize, _a1: S_Sandbox_StringToken) { hit(772);  }
unsafe extern "C" fn s773(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize) { hit(773);  }
unsafe extern "C" fn s774(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize) -> S_Matrix { hit(774); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s775(_a0: usize, _a1: S_Sandbox_StringToken) { hit(775);  }
unsafe extern "C" fn s776(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize) { hit(776);  }
unsafe extern "C" fn s777(_a0: usize, _a1: S_Sandbox_StringToken) { hit(777);  }
unsafe extern "C" fn s778(_a0: usize, _a1: S_Sandbox_StringToken, _a2: i32) { hit(778);  }
unsafe extern "C" fn s779(_a0: usize, _a1: S_Sandbox_StringToken, _a2: i32) -> i32 { hit(779); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s780(_a0: usize, _a1: S_Sandbox_StringToken) { hit(780);  }
unsafe extern "C" fn s781(_a0: usize, _a1: S_Sandbox_StringToken, _a2: u8) { hit(781);  }
unsafe extern "C" fn s782(_a0: usize, _a1: S_Sandbox_StringToken, _a2: u8) -> u8 { hit(782); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s783(_a0: usize, _a1: S_Sandbox_StringToken) { hit(783);  }
unsafe extern "C" fn s784(_a0: usize, _a1: S_Sandbox_StringToken, _a2: i32) { hit(784);  }
unsafe extern "C" fn s785(_a0: usize, _a1: S_Sandbox_StringToken, _a2: i32) -> i32 { hit(785); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s786(_a0: usize, _a1: S_Sandbox_StringToken) { hit(786);  }
unsafe extern "C" fn s787(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize, _a3: i32) { hit(787);  }
unsafe extern "C" fn s788(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize) -> usize { hit(788); crate::trace::object(788) }
unsafe extern "C" fn s789(_a0: usize, _a1: S_Sandbox_StringToken) { hit(789);  }
unsafe extern "C" fn s790(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize) { hit(790);  }
unsafe extern "C" fn s791(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize) { hit(791);  }
unsafe extern "C" fn s792(_a0: usize) { hit(792);  }
unsafe extern "C" fn s793(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize) { hit(793);  }
unsafe extern "C" fn s794(_a0: usize, _a1: S_Sandbox_StringToken) { hit(794);  }
unsafe extern "C" fn s795(_a0: usize, _a1: S_Sandbox_StringToken, _a2: i32, _a3: i32, _a4: i32, _a5: i32) { hit(795);  }
unsafe extern "C" fn s796(_a0: usize, _a1: usize) { hit(796);  }
unsafe extern "C" fn s797(_a0: usize) -> i32 { hit(797); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s798(_a0: usize, _a1: usize, _a2: i32) -> i32 { hit(798); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s799(_a0: usize, _a1: usize) { hit(799);  }
unsafe extern "C" fn s800(_a0: usize, _a1: i32, _a2: i32) { hit(800);  }
unsafe extern "C" fn s801(_a0: usize) { hit(801);  }
unsafe extern "C" fn s802(_a0: usize) -> i32 { hit(802); 1 }
unsafe extern "C" fn s803(_a0: usize) -> i32 { hit(803); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s804(_a0: usize) -> i32 { hit(804); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s805(_a0: usize) -> usize { hit(805); crate::trace::object(805) }
unsafe extern "C" fn s806(_a0: usize) -> usize { hit(806); crate::trace::object(806) }
unsafe extern "C" fn s807(_a0: usize) -> S_System_Guid { hit(807); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s808(_a0: usize) { hit(808);  }
unsafe extern "C" fn s809(_a0: usize) -> i32 { hit(809); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s810(_a0: usize) { hit(810);  }
unsafe extern "C" fn s811(_a0: usize, _a1: usize, _a2: usize) { hit(811);  }
unsafe extern "C" fn s812(_a0: usize, _a1: usize) { hit(812);  }
unsafe extern "C" fn s813(_a0: usize, _a1: S_Sandbox_StringToken) -> i32 { hit(813); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s814(_a0: usize) -> u32 { hit(814); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s815(_a0: usize) -> usize { hit(815); crate::trace::object(815) }
unsafe extern "C" fn s816(_a0: usize, _a1: usize) { hit(816);  }
unsafe extern "C" fn s817(_a0: usize) -> usize { hit(817); crate::trace::object(817) }
unsafe extern "C" fn s818(_a0: usize, _a1: usize) { hit(818);  }
unsafe extern "C" fn s819(_a0: usize) -> S_NativeEngine_RenderViewport { hit(819); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s820(_a0: usize, _a1: usize) { hit(820);  }
unsafe extern "C" fn s821(_a0: usize) -> S_NativeEngine_RenderViewport { hit(821); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s822(_a0: usize, _a1: usize) { hit(822);  }
unsafe extern "C" fn s823(_a0: usize) -> S_NativeEngine_RenderViewport { hit(823); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s824(_a0: usize, _a1: usize) { hit(824);  }
unsafe extern "C" fn s825(_a0: usize) -> usize { hit(825); crate::trace::object(825) }
unsafe extern "C" fn s826(_a0: usize, _a1: usize) { hit(826);  }
unsafe extern "C" fn s827(_a0: usize) -> usize { hit(827); crate::trace::object(827) }
unsafe extern "C" fn s828(_a0: usize, _a1: usize) { hit(828);  }
unsafe extern "C" fn s829(_a0: usize) -> usize { hit(829); crate::trace::object(829) }
unsafe extern "C" fn s830(_a0: usize, _a1: usize) { hit(830);  }
unsafe extern "C" fn s831(_a0: usize) -> usize { hit(831); crate::trace::object(831) }
unsafe extern "C" fn s832(_a0: usize, _a1: usize) { hit(832);  }
unsafe extern "C" fn s833(_a0: usize) -> i32 { hit(833); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s834(_a0: usize, _a1: i32) { hit(834);  }
unsafe extern "C" fn s835(_a0: usize) -> i32 { hit(835); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s836(_a0: usize, _a1: i32) { hit(836);  }
unsafe extern "C" fn s837(_a0: usize) -> i64 { hit(837); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s838(_a0: usize, _a1: i64) { hit(838);  }
unsafe extern "C" fn s839(_a0: usize) -> i64 { hit(839); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s840(_a0: usize, _a1: i64) { hit(840);  }
unsafe extern "C" fn s841(_a0: usize) -> usize { hit(841); crate::trace::object(841) }
unsafe extern "C" fn s842(_a0: usize, _a1: usize) { hit(842);  }
unsafe extern "C" fn s843(_a0: usize) -> i64 { hit(843); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s844(_a0: usize, _a1: i64) { hit(844);  }
unsafe extern "C" fn s845(_a0: usize) -> i64 { hit(845); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s846(_a0: usize, _a1: i64) { hit(846);  }
unsafe extern "C" fn s847(_a0: usize) -> i32 { hit(847); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s848(_a0: usize, _a1: i32) { hit(848);  }
unsafe extern "C" fn s849(_a0: usize) -> S_Vector4 { hit(849); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s850(_a0: usize, _a1: usize) { hit(850);  }
unsafe extern "C" fn s851(_a0: usize) -> i32 { hit(851); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s852(_a0: usize, _a1: i32) { hit(852);  }
unsafe extern "C" fn s853(_a0: usize) -> i32 { hit(853); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s854(_a0: usize, _a1: i32) { hit(854);  }
unsafe extern "C" fn s855(_a0: usize) -> i32 { hit(855); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s856(_a0: usize, _a1: i32) { hit(856);  }
unsafe extern "C" fn s857(_a0: usize) -> i32 { hit(857); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s858(_a0: usize, _a1: i32) { hit(858);  }
unsafe extern "C" fn s859(_a0: usize) -> i32 { hit(859); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s860(_a0: usize, _a1: i32) { hit(860);  }
unsafe extern "C" fn s861(_a0: usize) -> i32 { hit(861); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s862(_a0: usize, _a1: i32) { hit(862);  }
unsafe extern "C" fn s863(_a0: usize) -> i32 { hit(863); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s864(_a0: usize, _a1: i32) { hit(864);  }
unsafe extern "C" fn s865(_a0: usize) -> i32 { hit(865); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s866(_a0: usize, _a1: i32) { hit(866);  }
unsafe extern "C" fn s867(_a0: usize) -> i32 { hit(867); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s868(_a0: usize, _a1: i32) { hit(868);  }
unsafe extern "C" fn s869(_a0: usize) -> i32 { hit(869); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s870(_a0: usize, _a1: i32) { hit(870);  }
unsafe extern "C" fn s871(_a0: usize) -> i32 { hit(871); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s872(_a0: usize, _a1: i32) { hit(872);  }
unsafe extern "C" fn s873(_a0: usize) -> f32 { hit(873); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s874(_a0: usize, _a1: f32) { hit(874);  }
unsafe extern "C" fn s875(_a0: usize) -> i32 { hit(875); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s876(_a0: usize, _a1: i32) { hit(876);  }
unsafe extern "C" fn s877(_a0: usize) -> S_NativeEngine_RenderViewport { hit(877); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s878(_a0: usize, _a1: usize) { hit(878);  }
unsafe extern "C" fn s879(_a0: usize) -> i32 { hit(879); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s880(_a0: usize, _a1: i32) { hit(880);  }
unsafe extern "C" fn s881(_a0: usize) -> i32 { hit(881); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s882(_a0: usize, _a1: i32) { hit(882);  }
unsafe extern "C" fn s883(_a0: usize) -> usize { hit(883); crate::trace::object(883) }
unsafe extern "C" fn s884(_a0: usize, _a1: usize) { hit(884);  }
unsafe extern "C" fn s885(_a0: usize) -> usize { hit(885); crate::trace::object(885) }
unsafe extern "C" fn s886(_a0: usize, _a1: usize) { hit(886);  }
unsafe extern "C" fn s887(_a0: usize) -> usize { hit(887); crate::trace::object(887) }
unsafe extern "C" fn s888(_a0: usize, _a1: usize) { hit(888);  }
unsafe extern "C" fn s889(_a0: usize) -> f32 { hit(889); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s890(_a0: usize, _a1: f32) { hit(890);  }
unsafe extern "C" fn s891() -> usize { hit(891); crate::trace::object(891) }
unsafe extern "C" fn s892(_a0: usize) { hit(892);  }
unsafe extern "C" fn s893(_a0: usize, _a1: f32, _a2: f32, _a3: f32, _a4: usize, _a5: usize) { hit(893);  }
unsafe extern "C" fn s894(_a0: usize) -> usize { hit(894); _a0 }
unsafe extern "C" fn s895(_a0: usize) -> usize { hit(895); _a0 }
unsafe extern "C" fn s896(_a0: usize, _a1: i32, _a2: usize) { hit(896);  }
unsafe extern "C" fn s897(_a0: usize, _a1: i32, _a2: usize) { hit(897);  }
unsafe extern "C" fn s898(_a0: usize, _a1: i32, _a2: usize) { hit(898);  }
unsafe extern "C" fn s899(_a0: usize, _a1: i32) -> S_Transform { hit(899); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s900(_a0: usize, _a1: usize) -> S_Transform { hit(900); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s901(_a0: usize, _a1: i32) -> S_Transform { hit(901); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s902(_a0: usize, _a1: usize) -> S_Transform { hit(902); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s903(_a0: usize, _a1: i32) -> S_Transform { hit(903); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s904(_a0: usize, _a1: usize) -> S_Transform { hit(904); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s905(_a0: usize, _a1: i32) -> S_Transform { hit(905); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s906(_a0: usize, _a1: i32, _a2: usize) { hit(906);  }
unsafe extern "C" fn s907(_a0: usize, _a1: i32, _a2: usize) { hit(907);  }
unsafe extern "C" fn s908(_a0: usize, _a1: usize) -> i32 { hit(908); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s909(_a0: usize, _a1: f32) { hit(909);  }
unsafe extern "C" fn s910(_a0: usize, _a1: usize) { hit(910);  }
unsafe extern "C" fn s911(_a0: usize) { hit(911);  }
unsafe extern "C" fn s912(_a0: usize) { hit(912);  }
unsafe extern "C" fn s913(_a0: usize) { hit(913);  }
unsafe extern "C" fn s914(_a0: usize) { hit(914);  }
unsafe extern "C" fn s915(_a0: usize, _a1: i32) -> S_Transform { hit(915); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s916(_a0: usize, _a1: i32, _a2: usize) { hit(916);  }
unsafe extern "C" fn s917(_a0: usize, _a1: i32, _a2: usize) { hit(917);  }
unsafe extern "C" fn s918(_a0: usize, _a1: usize) { hit(918);  }
unsafe extern "C" fn s919(_a0: usize, _a1: usize) { hit(919);  }
unsafe extern "C" fn s920(_a0: usize, _a1: usize) { hit(920);  }
unsafe extern "C" fn s921(_a0: usize) -> usize { hit(921); crate::trace::object(921) }
unsafe extern "C" fn s922(_a0: usize, _a1: S_Sandbox_StringToken, _a2: f32, _a3: f32) { hit(922);  }
unsafe extern "C" fn s923(_a0: usize, _a1: i32, _a2: f32, _a3: f32) { hit(923);  }
unsafe extern "C" fn s924(_a0: usize, _a1: i32) -> f32 { hit(924); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s925(_a0: usize, _a1: S_Sandbox_StringToken) -> f32 { hit(925); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s926(_a0: usize, _a1: f32) { hit(926);  }
unsafe extern "C" fn s927(_a0: usize, _a1: S_Sandbox_StringToken, _a2: f32) { hit(927);  }
unsafe extern "C" fn s928(_a0: usize, _a1: i32, _a2: f32) { hit(928);  }
unsafe extern "C" fn s929(_a0: usize, _a1: usize) { hit(929);  }
unsafe extern "C" fn s930(_a0: usize, _a1: usize, _a2: usize, _a3: f32, _a4: f32) { hit(930);  }
unsafe extern "C" fn s931(_a0: usize) { hit(931);  }
unsafe extern "C" fn s932(_a0: usize) -> f32 { hit(932); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s933(_a0: usize) -> usize { hit(933); crate::trace::object(933) }
unsafe extern "C" fn s934(_a0: usize, _a1: f32) { hit(934);  }
unsafe extern "C" fn s935(_a0: usize) -> f32 { hit(935); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s936(_a0: usize, _a1: S_Sandbox_StringToken, _a2: i32, _a3: usize) -> i32 { hit(936); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s937(_a0: usize, _a1: i32) { hit(937);  }
unsafe extern "C" fn s938(_a0: usize) -> i32 { hit(938); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s939(_a0: usize) -> usize { hit(939); crate::trace::object(939) }
unsafe extern "C" fn s940(_a0: usize, _a1: usize) { hit(940);  }
unsafe extern "C" fn s941(_a0: usize) -> f32 { hit(941); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s942(_a0: usize) -> f32 { hit(942); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s943(_a0: usize, _a1: f32) { hit(943);  }
unsafe extern "C" fn s944(_a0: usize, _a1: i32) { hit(944);  }
unsafe extern "C" fn s945(_a0: usize) -> i32 { hit(945); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s946(_a0: usize, _a1: i32) { hit(946);  }
unsafe extern "C" fn s947(_a0: usize) -> f32 { hit(947); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s948(_a0: usize, _a1: f32) { hit(948);  }
unsafe extern "C" fn s949(_a0: usize) -> i32 { hit(949); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s950(_a0: usize, _a1: usize) { hit(950);  }
unsafe extern "C" fn s951(_a0: usize, _a1: usize) { hit(951);  }
unsafe extern "C" fn s952(_a0: usize) { hit(952);  }
unsafe extern "C" fn s953(_a0: usize, _a1: u16, _a2: usize) { hit(953);  }
unsafe extern "C" fn s954(_a0: usize) -> S_Transform { hit(954); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s955(_a0: usize) -> i32 { hit(955); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s956(_a0: usize, _a1: usize) -> usize { hit(956); crate::trace::object(956) }
unsafe extern "C" fn s957(_a0: usize, _a1: i32) -> usize { hit(957); crate::trace::object(957) }
unsafe extern "C" fn s958(_a0: usize, _a1: i64, _a2: i64) { hit(958);  }
unsafe extern "C" fn s959(_a0: usize, _a1: i64) { hit(959);  }
unsafe extern "C" fn s960(_a0: usize, _a1: i64) -> i32 { hit(960); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s961(_a0: usize) -> i64 { hit(961); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s962(_a0: usize) -> i64 { hit(962); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s963(_a0: usize, _a1: i64) { hit(963);  }
unsafe extern "C" fn s964(_a0: usize, _a1: f32) { hit(964);  }
unsafe extern "C" fn s965(_a0: usize) { hit(965);  }
unsafe extern "C" fn s966(_a0: usize, _a1: usize, _a2: i32) { hit(966);  }
unsafe extern "C" fn s967(_a0: usize) -> S_Vector3 { hit(967); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s968(_a0: usize) -> i32 { hit(968); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s969(_a0: usize, _a1: usize) { hit(969);  }
unsafe extern "C" fn s970(_a0: usize) -> S_Vector4 { hit(970); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s971(_a0: usize, _a1: f32) { hit(971);  }
unsafe extern "C" fn s972(_a0: usize) -> f32 { hit(972); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s973(_a0: usize, _a1: usize) { hit(973);  }
unsafe extern "C" fn s974(_a0: usize) { hit(974);  }
unsafe extern "C" fn s975(_a0: usize, _a1: usize, _a2: S_Sandbox_StringToken, _a3: i32) { hit(975);  }
unsafe extern "C" fn s976(_a0: usize) -> i32 { hit(976); 1 }
unsafe extern "C" fn s977(_a0: usize) -> i32 { hit(977); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s978(_a0: usize) { hit(978);  }
unsafe extern "C" fn s979(_a0: usize) { hit(979);  }
unsafe extern "C" fn s980(_a0: usize) { hit(980);  }
unsafe extern "C" fn s981(_a0: usize) { hit(981);  }
unsafe extern "C" fn s982(_a0: usize, _a1: i32) { hit(982);  }
unsafe extern "C" fn s983(_a0: usize) -> f32 { hit(983); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s984(_a0: usize, _a1: usize) { hit(984);  }
unsafe extern "C" fn s985(_a0: usize) -> S_Transform { hit(985); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s986(_a0: usize, _a1: usize) { hit(986);  }
unsafe extern "C" fn s987(_a0: usize) -> S_BBox { hit(987); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s988(_a0: usize) { hit(988);  }
unsafe extern "C" fn s989(_a0: usize) -> i32 { hit(989); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s990(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize, _a3: u32) { hit(990);  }
unsafe extern "C" fn s991(_a0: usize, _a1: usize) { hit(991);  }
unsafe extern "C" fn s992(_a0: usize) -> usize { hit(992); crate::trace::object(992) }
unsafe extern "C" fn s993(_a0: usize, _a1: u64) { hit(993);  }
unsafe extern "C" fn s994(_a0: usize, _a1: u64) { hit(994);  }
unsafe extern "C" fn s995(_a0: usize, _a1: u64) { hit(995);  }
unsafe extern "C" fn s996(_a0: usize) -> u64 { hit(996); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s997(_a0: usize) -> i32 { hit(997); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s998(_a0: usize, _a1: i32) { hit(998);  }
unsafe extern "C" fn s999(_a0: usize) { hit(999);  }
unsafe extern "C" fn s1000(_a0: usize) -> u64 { hit(1000); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1001(_a0: usize) -> i32 { hit(1001); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1002(_a0: usize) -> usize { hit(1002); crate::trace::object(1002) }
unsafe extern "C" fn s1003(_a0: usize, _a1: S_Sandbox_StringToken) { hit(1003);  }
unsafe extern "C" fn s1004(_a0: usize, _a1: S_Sandbox_StringToken, _a2: i32) { hit(1004);  }
unsafe extern "C" fn s1005(_a0: usize, _a1: i32) { hit(1005);  }
unsafe extern "C" fn s1006(_a0: usize) -> i32 { hit(1006); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1007(_a0: usize, _a1: i32) { hit(1007);  }
unsafe extern "C" fn s1008(_a0: usize, _a1: u32) { hit(1008);  }
unsafe extern "C" fn s1009(_a0: usize) { hit(1009);  }
unsafe extern "C" fn s1010(_a0: usize) -> i32 { hit(1010); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1011(_a0: usize, _a1: i32) -> u32 { hit(1011); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1012(_a0: usize, _a1: u32) { hit(1012);  }
unsafe extern "C" fn s1013(_a0: usize, _a1: u32) -> i32 { hit(1013); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1014(_a0: usize, _a1: S_Sandbox_StringToken) { hit(1014);  }
unsafe extern "C" fn s1015(_a0: usize, _a1: S_Sandbox_StringToken) { hit(1015);  }
unsafe extern "C" fn s1016(_a0: usize) -> u32 { hit(1016); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1017(_a0: usize, _a1: S_Sandbox_StringToken, _a2: i32) -> i32 { hit(1017); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1018(_a0: usize) { hit(1018);  }
unsafe extern "C" fn s1019(_a0: usize, _a1: i32, _a2: usize) { hit(1019);  }
unsafe extern "C" fn s1020(_a0: usize) -> f32 { hit(1020); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1021(_a0: usize, _a1: f32) { hit(1021);  }
unsafe extern "C" fn s1022(_a0: usize) -> S_BBox { hit(1022); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1023(_a0: usize, _a1: usize) { hit(1023);  }
unsafe extern "C" fn s1024(_a0: usize) -> S_BBox { hit(1024); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1025(_a0: usize, _a1: usize) { hit(1025);  }
unsafe extern "C" fn s1026(_a0: usize) -> usize { hit(1026); _a0 }
unsafe extern "C" fn s1027(_a0: usize) -> usize { hit(1027); _a0 }
unsafe extern "C" fn s1028(_a0: usize, _a1: usize) { hit(1028);  }
unsafe extern "C" fn s1029(_a0: usize) -> S_Vector3 { hit(1029); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1030(_a0: usize, _a1: usize) { hit(1030);  }
unsafe extern "C" fn s1031(_a0: usize) -> S_Vector3 { hit(1031); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1032(_a0: usize, _a1: usize) { hit(1032);  }
unsafe extern "C" fn s1033(_a0: usize, _a1: usize) { hit(1033);  }
unsafe extern "C" fn s1034(_a0: usize) -> S_Vector3 { hit(1034); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1035(_a0: usize, _a1: f32) { hit(1035);  }
unsafe extern "C" fn s1036(_a0: usize) -> f32 { hit(1036); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1037(_a0: usize, _a1: f32) { hit(1037);  }
unsafe extern "C" fn s1038(_a0: usize) -> f32 { hit(1038); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1039(_a0: usize, _a1: f32) { hit(1039);  }
unsafe extern "C" fn s1040(_a0: usize) -> f32 { hit(1040); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1041(_a0: usize, _a1: f32) { hit(1041);  }
unsafe extern "C" fn s1042(_a0: usize) -> f32 { hit(1042); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1043(_a0: usize) -> i32 { hit(1043); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1044(_a0: usize, _a1: i32) { hit(1044);  }
unsafe extern "C" fn s1045(_a0: usize) -> i32 { hit(1045); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1046(_a0: usize, _a1: i32) { hit(1046);  }
unsafe extern "C" fn s1047(_a0: usize, _a1: f32) { hit(1047);  }
unsafe extern "C" fn s1048(_a0: usize) -> f32 { hit(1048); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1049(_a0: usize, _a1: f32) { hit(1049);  }
unsafe extern "C" fn s1050(_a0: usize) -> f32 { hit(1050); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1051(_a0: usize, _a1: f32) { hit(1051);  }
unsafe extern "C" fn s1052(_a0: usize) -> f32 { hit(1052); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1053(_a0: usize, _a1: usize) { hit(1053);  }
unsafe extern "C" fn s1054(_a0: usize) -> usize { hit(1054); crate::trace::object(1054) }
unsafe extern "C" fn s1055(_a0: usize, _a1: u32, _a2: u32) { hit(1055);  }
unsafe extern "C" fn s1056(_a0: usize) -> i32 { hit(1056); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1057(_a0: usize, _a1: i32) { hit(1057);  }
unsafe extern "C" fn s1058(_a0: usize) -> f32 { hit(1058); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1059(_a0: usize, _a1: f32) { hit(1059);  }
unsafe extern "C" fn s1060(_a0: usize) -> f32 { hit(1060); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1061(_a0: usize, _a1: f32) { hit(1061);  }
unsafe extern "C" fn s1062(_a0: usize) -> f32 { hit(1062); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1063(_a0: usize, _a1: f32) { hit(1063);  }
unsafe extern "C" fn s1064(_a0: usize) -> i32 { hit(1064); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1065(_a0: usize, _a1: i32) { hit(1065);  }
unsafe extern "C" fn s1066(_a0: usize, _a1: i32) { hit(1066);  }
unsafe extern "C" fn s1067(_a0: usize, _a1: f32) { hit(1067);  }
unsafe extern "C" fn s1068(_a0: usize, _a1: i32) { hit(1068);  }
unsafe extern "C" fn s1069(_a0: usize, _a1: i32) { hit(1069);  }
unsafe extern "C" fn s1070(_a0: usize, _a1: i32) { hit(1070);  }
unsafe extern "C" fn s1071(_a0: usize, _a1: i32) { hit(1071);  }
unsafe extern "C" fn s1072(_a0: usize, _a1: f32) { hit(1072);  }
unsafe extern "C" fn s1073(_a0: usize, _a1: f32) { hit(1073);  }
unsafe extern "C" fn s1074(_a0: usize, _a1: i32) { hit(1074);  }
unsafe extern "C" fn s1075(_a0: usize, _a1: i32) { hit(1075);  }
unsafe extern "C" fn s1076(_a0: usize) -> i32 { hit(1076); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1077(_a0: usize) -> i32 { hit(1077); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1078(_a0: usize) -> u32 { hit(1078); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1079(_a0: usize, _a1: u32) { hit(1079);  }
unsafe extern "C" fn s1080(_a0: usize) -> i64 { hit(1080); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1081(_a0: usize, _a1: i64) { hit(1081);  }
unsafe extern "C" fn s1082(_a0: usize, _a1: f32) { hit(1082);  }
unsafe extern "C" fn s1083(_a0: usize, _a1: f32) { hit(1083);  }
unsafe extern "C" fn s1084(_a0: usize) -> i32 { hit(1084); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1085(_a0: usize, _a1: i64, _a2: i64) { hit(1085);  }
unsafe extern "C" fn s1086(_a0: usize, _a1: i64) { hit(1086);  }
unsafe extern "C" fn s1087(_a0: usize, _a1: i64) -> i32 { hit(1087); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1088(_a0: usize) -> i64 { hit(1088); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1089(_a0: usize) -> i64 { hit(1089); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1090(_a0: usize, _a1: i64) { hit(1090);  }
unsafe extern "C" fn s1091(_a0: usize, _a1: f32) { hit(1091);  }
unsafe extern "C" fn s1092(_a0: usize) { hit(1092);  }
unsafe extern "C" fn s1093(_a0: usize, _a1: usize, _a2: i32) { hit(1093);  }
unsafe extern "C" fn s1094(_a0: usize) -> S_Vector3 { hit(1094); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1095(_a0: usize) -> i32 { hit(1095); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1096(_a0: usize, _a1: usize) { hit(1096);  }
unsafe extern "C" fn s1097(_a0: usize) -> S_Vector4 { hit(1097); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1098(_a0: usize, _a1: f32) { hit(1098);  }
unsafe extern "C" fn s1099(_a0: usize) -> f32 { hit(1099); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1100(_a0: usize, _a1: usize) { hit(1100);  }
unsafe extern "C" fn s1101(_a0: usize) { hit(1101);  }
unsafe extern "C" fn s1102(_a0: usize, _a1: usize, _a2: S_Sandbox_StringToken, _a3: i32) { hit(1102);  }
unsafe extern "C" fn s1103(_a0: usize) -> i32 { hit(1103); 1 }
unsafe extern "C" fn s1104(_a0: usize) -> i32 { hit(1104); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1105(_a0: usize) { hit(1105);  }
unsafe extern "C" fn s1106(_a0: usize) { hit(1106);  }
unsafe extern "C" fn s1107(_a0: usize) { hit(1107);  }
unsafe extern "C" fn s1108(_a0: usize) { hit(1108);  }
unsafe extern "C" fn s1109(_a0: usize, _a1: i32) { hit(1109);  }
unsafe extern "C" fn s1110(_a0: usize) -> f32 { hit(1110); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1111(_a0: usize, _a1: usize) { hit(1111);  }
unsafe extern "C" fn s1112(_a0: usize) -> S_Transform { hit(1112); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1113(_a0: usize, _a1: usize) { hit(1113);  }
unsafe extern "C" fn s1114(_a0: usize) -> S_BBox { hit(1114); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1115(_a0: usize) { hit(1115);  }
unsafe extern "C" fn s1116(_a0: usize) -> i32 { hit(1116); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1117(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize, _a3: u32) { hit(1117);  }
unsafe extern "C" fn s1118(_a0: usize, _a1: usize) { hit(1118);  }
unsafe extern "C" fn s1119(_a0: usize) -> usize { hit(1119); crate::trace::object(1119) }
unsafe extern "C" fn s1120(_a0: usize, _a1: u64) { hit(1120);  }
unsafe extern "C" fn s1121(_a0: usize, _a1: u64) { hit(1121);  }
unsafe extern "C" fn s1122(_a0: usize, _a1: u64) { hit(1122);  }
unsafe extern "C" fn s1123(_a0: usize) -> u64 { hit(1123); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1124(_a0: usize) -> i32 { hit(1124); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1125(_a0: usize, _a1: i32) { hit(1125);  }
unsafe extern "C" fn s1126(_a0: usize) { hit(1126);  }
unsafe extern "C" fn s1127(_a0: usize) -> u64 { hit(1127); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1128(_a0: usize) -> i32 { hit(1128); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1129(_a0: usize) -> usize { hit(1129); crate::trace::object(1129) }
unsafe extern "C" fn s1130(_a0: usize, _a1: S_Sandbox_StringToken) { hit(1130);  }
unsafe extern "C" fn s1131(_a0: usize, _a1: S_Sandbox_StringToken, _a2: i32) { hit(1131);  }
unsafe extern "C" fn s1132(_a0: usize, _a1: i32) { hit(1132);  }
unsafe extern "C" fn s1133(_a0: usize) -> i32 { hit(1133); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1134(_a0: usize, _a1: i32) { hit(1134);  }
unsafe extern "C" fn s1135(_a0: usize, _a1: u32) { hit(1135);  }
unsafe extern "C" fn s1136(_a0: usize) { hit(1136);  }
unsafe extern "C" fn s1137(_a0: usize) -> i32 { hit(1137); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1138(_a0: usize, _a1: i32) -> u32 { hit(1138); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1139(_a0: usize, _a1: u32) { hit(1139);  }
unsafe extern "C" fn s1140(_a0: usize, _a1: u32) -> i32 { hit(1140); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1141(_a0: usize, _a1: S_Sandbox_StringToken) { hit(1141);  }
unsafe extern "C" fn s1142(_a0: usize, _a1: S_Sandbox_StringToken) { hit(1142);  }
unsafe extern "C" fn s1143(_a0: usize) -> u32 { hit(1143); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1144(_a0: usize, _a1: S_Sandbox_StringToken, _a2: i32) -> i32 { hit(1144); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1145(_a0: usize) { hit(1145);  }
unsafe extern "C" fn s1146(_a0: usize, _a1: i32, _a2: usize) { hit(1146);  }
unsafe extern "C" fn s1147(_a0: usize) -> usize { hit(1147); _a0 }
unsafe extern "C" fn s1148(_a0: usize) -> usize { hit(1148); _a0 }
unsafe extern "C" fn s1149(_a0: usize) { hit(1149);  }
unsafe extern "C" fn s1150(_a0: usize, _a1: i64, _a2: i64) { hit(1150);  }
unsafe extern "C" fn s1151(_a0: usize, _a1: i64) { hit(1151);  }
unsafe extern "C" fn s1152(_a0: usize, _a1: i64) -> i32 { hit(1152); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1153(_a0: usize) -> i64 { hit(1153); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1154(_a0: usize) -> i64 { hit(1154); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1155(_a0: usize, _a1: i64) { hit(1155);  }
unsafe extern "C" fn s1156(_a0: usize, _a1: f32) { hit(1156);  }
unsafe extern "C" fn s1157(_a0: usize) { hit(1157);  }
unsafe extern "C" fn s1158(_a0: usize, _a1: usize, _a2: i32) { hit(1158);  }
unsafe extern "C" fn s1159(_a0: usize) -> S_Vector3 { hit(1159); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1160(_a0: usize) -> i32 { hit(1160); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1161(_a0: usize, _a1: usize) { hit(1161);  }
unsafe extern "C" fn s1162(_a0: usize) -> S_Vector4 { hit(1162); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1163(_a0: usize, _a1: f32) { hit(1163);  }
unsafe extern "C" fn s1164(_a0: usize) -> f32 { hit(1164); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1165(_a0: usize, _a1: usize) { hit(1165);  }
unsafe extern "C" fn s1166(_a0: usize) { hit(1166);  }
unsafe extern "C" fn s1167(_a0: usize, _a1: usize, _a2: S_Sandbox_StringToken, _a3: i32) { hit(1167);  }
unsafe extern "C" fn s1168(_a0: usize) -> i32 { hit(1168); 1 }
unsafe extern "C" fn s1169(_a0: usize) -> i32 { hit(1169); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1170(_a0: usize) { hit(1170);  }
unsafe extern "C" fn s1171(_a0: usize) { hit(1171);  }
unsafe extern "C" fn s1172(_a0: usize) { hit(1172);  }
unsafe extern "C" fn s1173(_a0: usize) { hit(1173);  }
unsafe extern "C" fn s1174(_a0: usize, _a1: i32) { hit(1174);  }
unsafe extern "C" fn s1175(_a0: usize) -> f32 { hit(1175); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1176(_a0: usize, _a1: usize) { hit(1176);  }
unsafe extern "C" fn s1177(_a0: usize) -> S_Transform { hit(1177); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1178(_a0: usize, _a1: usize) { hit(1178);  }
unsafe extern "C" fn s1179(_a0: usize) -> S_BBox { hit(1179); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1180(_a0: usize) { hit(1180);  }
unsafe extern "C" fn s1181(_a0: usize) -> i32 { hit(1181); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1182(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize, _a3: u32) { hit(1182);  }
unsafe extern "C" fn s1183(_a0: usize, _a1: usize) { hit(1183);  }
unsafe extern "C" fn s1184(_a0: usize) -> usize { hit(1184); crate::trace::object(1184) }
unsafe extern "C" fn s1185(_a0: usize, _a1: u64) { hit(1185);  }
unsafe extern "C" fn s1186(_a0: usize, _a1: u64) { hit(1186);  }
unsafe extern "C" fn s1187(_a0: usize, _a1: u64) { hit(1187);  }
unsafe extern "C" fn s1188(_a0: usize) -> u64 { hit(1188); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1189(_a0: usize) -> i32 { hit(1189); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1190(_a0: usize, _a1: i32) { hit(1190);  }
unsafe extern "C" fn s1191(_a0: usize) { hit(1191);  }
unsafe extern "C" fn s1192(_a0: usize) -> u64 { hit(1192); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1193(_a0: usize) -> i32 { hit(1193); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1194(_a0: usize) -> usize { hit(1194); crate::trace::object(1194) }
unsafe extern "C" fn s1195(_a0: usize, _a1: S_Sandbox_StringToken) { hit(1195);  }
unsafe extern "C" fn s1196(_a0: usize, _a1: S_Sandbox_StringToken, _a2: i32) { hit(1196);  }
unsafe extern "C" fn s1197(_a0: usize, _a1: i32) { hit(1197);  }
unsafe extern "C" fn s1198(_a0: usize) -> i32 { hit(1198); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1199(_a0: usize, _a1: i32) { hit(1199);  }
unsafe extern "C" fn s1200(_a0: usize, _a1: u32) { hit(1200);  }
unsafe extern "C" fn s1201(_a0: usize) { hit(1201);  }
unsafe extern "C" fn s1202(_a0: usize) -> i32 { hit(1202); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1203(_a0: usize, _a1: i32) -> u32 { hit(1203); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1204(_a0: usize, _a1: u32) { hit(1204);  }
unsafe extern "C" fn s1205(_a0: usize, _a1: u32) -> i32 { hit(1205); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1206(_a0: usize, _a1: S_Sandbox_StringToken) { hit(1206);  }
unsafe extern "C" fn s1207(_a0: usize, _a1: S_Sandbox_StringToken) { hit(1207);  }
unsafe extern "C" fn s1208(_a0: usize) -> u32 { hit(1208); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1209(_a0: usize, _a1: S_Sandbox_StringToken, _a2: i32) -> i32 { hit(1209); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1210(_a0: usize) { hit(1210);  }
unsafe extern "C" fn s1211(_a0: usize, _a1: i32, _a2: usize) { hit(1211);  }
unsafe extern "C" fn s1212(_a0: usize) -> S_Vector3 { hit(1212); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1213(_a0: usize, _a1: usize) { hit(1213);  }
unsafe extern "C" fn s1214(_a0: usize) -> S_Vector3 { hit(1214); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1215(_a0: usize, _a1: usize) { hit(1215);  }
unsafe extern "C" fn s1216(_a0: usize) -> i32 { hit(1216); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1217(_a0: usize, _a1: i32) { hit(1217);  }
unsafe extern "C" fn s1218(_a0: usize) -> i32 { hit(1218); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1219(_a0: usize, _a1: i32) { hit(1219);  }
unsafe extern "C" fn s1220(_a0: usize) -> usize { hit(1220); crate::trace::object(1220) }
unsafe extern "C" fn s1221(_a0: usize, _a1: usize) { hit(1221);  }
unsafe extern "C" fn s1222(_a0: usize) -> usize { hit(1222); crate::trace::object(1222) }
unsafe extern "C" fn s1223(_a0: usize, _a1: usize) { hit(1223);  }
unsafe extern "C" fn s1224(_a0: usize) -> usize { hit(1224); crate::trace::object(1224) }
unsafe extern "C" fn s1225(_a0: usize, _a1: usize) { hit(1225);  }
unsafe extern "C" fn s1226(_a0: usize, _a1: i64, _a2: i64) { hit(1226);  }
unsafe extern "C" fn s1227(_a0: usize, _a1: i64) { hit(1227);  }
unsafe extern "C" fn s1228(_a0: usize, _a1: i64) -> i32 { hit(1228); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1229(_a0: usize) -> i64 { hit(1229); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1230(_a0: usize) -> i64 { hit(1230); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1231(_a0: usize, _a1: i64) { hit(1231);  }
unsafe extern "C" fn s1232(_a0: usize, _a1: f32) { hit(1232);  }
unsafe extern "C" fn s1233(_a0: usize) { hit(1233);  }
unsafe extern "C" fn s1234(_a0: usize, _a1: usize, _a2: i32) { hit(1234);  }
unsafe extern "C" fn s1235(_a0: usize) -> S_Vector3 { hit(1235); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1236(_a0: usize) -> i32 { hit(1236); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1237(_a0: usize, _a1: usize) { hit(1237);  }
unsafe extern "C" fn s1238(_a0: usize) -> S_Vector4 { hit(1238); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1239(_a0: usize, _a1: f32) { hit(1239);  }
unsafe extern "C" fn s1240(_a0: usize) -> f32 { hit(1240); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1241(_a0: usize, _a1: usize) { hit(1241);  }
unsafe extern "C" fn s1242(_a0: usize) { hit(1242);  }
unsafe extern "C" fn s1243(_a0: usize, _a1: usize, _a2: S_Sandbox_StringToken, _a3: i32) { hit(1243);  }
unsafe extern "C" fn s1244(_a0: usize) -> i32 { hit(1244); 1 }
unsafe extern "C" fn s1245(_a0: usize) -> i32 { hit(1245); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1246(_a0: usize) { hit(1246);  }
unsafe extern "C" fn s1247(_a0: usize) { hit(1247);  }
unsafe extern "C" fn s1248(_a0: usize) { hit(1248);  }
unsafe extern "C" fn s1249(_a0: usize) { hit(1249);  }
unsafe extern "C" fn s1250(_a0: usize, _a1: i32) { hit(1250);  }
unsafe extern "C" fn s1251(_a0: usize) -> f32 { hit(1251); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1252(_a0: usize, _a1: usize) { hit(1252);  }
unsafe extern "C" fn s1253(_a0: usize) -> S_Transform { hit(1253); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1254(_a0: usize, _a1: usize) { hit(1254);  }
unsafe extern "C" fn s1255(_a0: usize) -> S_BBox { hit(1255); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1256(_a0: usize) { hit(1256);  }
unsafe extern "C" fn s1257(_a0: usize) -> i32 { hit(1257); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1258(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize, _a3: u32) { hit(1258);  }
unsafe extern "C" fn s1259(_a0: usize, _a1: usize) { hit(1259);  }
unsafe extern "C" fn s1260(_a0: usize) -> usize { hit(1260); crate::trace::object(1260) }
unsafe extern "C" fn s1261(_a0: usize, _a1: u64) { hit(1261);  }
unsafe extern "C" fn s1262(_a0: usize, _a1: u64) { hit(1262);  }
unsafe extern "C" fn s1263(_a0: usize, _a1: u64) { hit(1263);  }
unsafe extern "C" fn s1264(_a0: usize) -> u64 { hit(1264); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1265(_a0: usize) -> i32 { hit(1265); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1266(_a0: usize, _a1: i32) { hit(1266);  }
unsafe extern "C" fn s1267(_a0: usize) { hit(1267);  }
unsafe extern "C" fn s1268(_a0: usize) -> u64 { hit(1268); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1269(_a0: usize) -> i32 { hit(1269); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1270(_a0: usize) -> usize { hit(1270); crate::trace::object(1270) }
unsafe extern "C" fn s1271(_a0: usize, _a1: S_Sandbox_StringToken) { hit(1271);  }
unsafe extern "C" fn s1272(_a0: usize, _a1: S_Sandbox_StringToken, _a2: i32) { hit(1272);  }
unsafe extern "C" fn s1273(_a0: usize, _a1: i32) { hit(1273);  }
unsafe extern "C" fn s1274(_a0: usize) -> i32 { hit(1274); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1275(_a0: usize, _a1: i32) { hit(1275);  }
unsafe extern "C" fn s1276(_a0: usize, _a1: u32) { hit(1276);  }
unsafe extern "C" fn s1277(_a0: usize) { hit(1277);  }
unsafe extern "C" fn s1278(_a0: usize) -> i32 { hit(1278); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1279(_a0: usize, _a1: i32) -> u32 { hit(1279); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1280(_a0: usize, _a1: u32) { hit(1280);  }
unsafe extern "C" fn s1281(_a0: usize, _a1: u32) -> i32 { hit(1281); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1282(_a0: usize, _a1: S_Sandbox_StringToken) { hit(1282);  }
unsafe extern "C" fn s1283(_a0: usize, _a1: S_Sandbox_StringToken) { hit(1283);  }
unsafe extern "C" fn s1284(_a0: usize) -> u32 { hit(1284); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1285(_a0: usize, _a1: S_Sandbox_StringToken, _a2: i32) -> i32 { hit(1285); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1286(_a0: usize) { hit(1286);  }
unsafe extern "C" fn s1287(_a0: usize, _a1: i32, _a2: usize) { hit(1287);  }
unsafe extern "C" fn s1288(_a0: usize) -> usize { hit(1288); _a0 }
unsafe extern "C" fn s1289(_a0: usize) -> usize { hit(1289); _a0 }
unsafe extern "C" fn s1290(_a0: usize, _a1: usize) { hit(1290);  }
unsafe extern "C" fn s1291(_a0: usize, _a1: usize, _a2: usize, _a3: i32) { hit(1291);  }
unsafe extern "C" fn s1292(_a0: usize) -> usize { hit(1292); crate::trace::object(1292) }
unsafe extern "C" fn s1293(_a0: usize, _a1: usize) { hit(1293);  }
unsafe extern "C" fn s1294(_a0: usize, _a1: usize) { hit(1294);  }
unsafe extern "C" fn s1295(_a0: usize) -> S_Vector3 { hit(1295); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1296(_a0: usize, _a1: i32) { hit(1296);  }
unsafe extern "C" fn s1297(_a0: usize) -> i32 { hit(1297); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1298(_a0: usize, _a1: f32, _a2: f32, _a3: f32, _a4: f32) { hit(1298);  }
unsafe extern "C" fn s1299(_a0: usize) -> f32 { hit(1299); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1300(_a0: usize) -> f32 { hit(1300); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1301(_a0: usize) -> f32 { hit(1301); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1302(_a0: usize) -> f32 { hit(1302); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1303(_a0: usize, _a1: i64, _a2: i64) { hit(1303);  }
unsafe extern "C" fn s1304(_a0: usize, _a1: i64) { hit(1304);  }
unsafe extern "C" fn s1305(_a0: usize, _a1: i64) -> i32 { hit(1305); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1306(_a0: usize) -> i64 { hit(1306); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1307(_a0: usize) -> i64 { hit(1307); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1308(_a0: usize, _a1: i64) { hit(1308);  }
unsafe extern "C" fn s1309(_a0: usize, _a1: f32) { hit(1309);  }
unsafe extern "C" fn s1310(_a0: usize) { hit(1310);  }
unsafe extern "C" fn s1311(_a0: usize, _a1: usize, _a2: i32) { hit(1311);  }
unsafe extern "C" fn s1312(_a0: usize) -> S_Vector3 { hit(1312); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1313(_a0: usize) -> i32 { hit(1313); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1314(_a0: usize, _a1: usize) { hit(1314);  }
unsafe extern "C" fn s1315(_a0: usize) -> S_Vector4 { hit(1315); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1316(_a0: usize, _a1: f32) { hit(1316);  }
unsafe extern "C" fn s1317(_a0: usize) -> f32 { hit(1317); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1318(_a0: usize, _a1: usize) { hit(1318);  }
unsafe extern "C" fn s1319(_a0: usize) { hit(1319);  }
unsafe extern "C" fn s1320(_a0: usize, _a1: usize, _a2: S_Sandbox_StringToken, _a3: i32) { hit(1320);  }
unsafe extern "C" fn s1321(_a0: usize) -> i32 { hit(1321); 1 }
unsafe extern "C" fn s1322(_a0: usize) -> i32 { hit(1322); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1323(_a0: usize) { hit(1323);  }
unsafe extern "C" fn s1324(_a0: usize) { hit(1324);  }
unsafe extern "C" fn s1325(_a0: usize) { hit(1325);  }
unsafe extern "C" fn s1326(_a0: usize) { hit(1326);  }
unsafe extern "C" fn s1327(_a0: usize, _a1: i32) { hit(1327);  }
unsafe extern "C" fn s1328(_a0: usize) -> f32 { hit(1328); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1329(_a0: usize, _a1: usize) { hit(1329);  }
unsafe extern "C" fn s1330(_a0: usize) -> S_Transform { hit(1330); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1331(_a0: usize, _a1: usize) { hit(1331);  }
unsafe extern "C" fn s1332(_a0: usize) -> S_BBox { hit(1332); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1333(_a0: usize) { hit(1333);  }
unsafe extern "C" fn s1334(_a0: usize) -> i32 { hit(1334); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1335(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize, _a3: u32) { hit(1335);  }
unsafe extern "C" fn s1336(_a0: usize, _a1: usize) { hit(1336);  }
unsafe extern "C" fn s1337(_a0: usize) -> usize { hit(1337); crate::trace::object(1337) }
unsafe extern "C" fn s1338(_a0: usize, _a1: u64) { hit(1338);  }
unsafe extern "C" fn s1339(_a0: usize, _a1: u64) { hit(1339);  }
unsafe extern "C" fn s1340(_a0: usize, _a1: u64) { hit(1340);  }
unsafe extern "C" fn s1341(_a0: usize) -> u64 { hit(1341); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1342(_a0: usize) -> i32 { hit(1342); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1343(_a0: usize, _a1: i32) { hit(1343);  }
unsafe extern "C" fn s1344(_a0: usize) { hit(1344);  }
unsafe extern "C" fn s1345(_a0: usize) -> u64 { hit(1345); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1346(_a0: usize) -> i32 { hit(1346); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1347(_a0: usize) -> usize { hit(1347); crate::trace::object(1347) }
unsafe extern "C" fn s1348(_a0: usize, _a1: S_Sandbox_StringToken) { hit(1348);  }
unsafe extern "C" fn s1349(_a0: usize, _a1: S_Sandbox_StringToken, _a2: i32) { hit(1349);  }
unsafe extern "C" fn s1350(_a0: usize, _a1: i32) { hit(1350);  }
unsafe extern "C" fn s1351(_a0: usize) -> i32 { hit(1351); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1352(_a0: usize, _a1: i32) { hit(1352);  }
unsafe extern "C" fn s1353(_a0: usize, _a1: u32) { hit(1353);  }
unsafe extern "C" fn s1354(_a0: usize) { hit(1354);  }
unsafe extern "C" fn s1355(_a0: usize) -> i32 { hit(1355); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1356(_a0: usize, _a1: i32) -> u32 { hit(1356); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1357(_a0: usize, _a1: u32) { hit(1357);  }
unsafe extern "C" fn s1358(_a0: usize, _a1: u32) -> i32 { hit(1358); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1359(_a0: usize, _a1: S_Sandbox_StringToken) { hit(1359);  }
unsafe extern "C" fn s1360(_a0: usize, _a1: S_Sandbox_StringToken) { hit(1360);  }
unsafe extern "C" fn s1361(_a0: usize) -> u32 { hit(1361); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1362(_a0: usize, _a1: S_Sandbox_StringToken, _a2: i32) -> i32 { hit(1362); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1363(_a0: usize) { hit(1363);  }
unsafe extern "C" fn s1364(_a0: usize, _a1: i32, _a2: usize) { hit(1364);  }
unsafe extern "C" fn s1365(_a0: u32) -> usize { hit(1365); crate::trace::object(1365) }
unsafe extern "C" fn s1366(_a0: usize) { hit(1366);  }
unsafe extern "C" fn s1367(_a0: usize) { hit(1367);  }
unsafe extern "C" fn s1368(_a0: usize, _a1: usize, _a2: usize) { hit(1368);  }
unsafe extern "C" fn s1369(_a0: usize) -> i32 { hit(1369); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1370(_a0: usize) -> i32 { hit(1370); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1371(_a0: usize, _a1: usize, _a2: u32) -> i32 { hit(1371); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1372(_a0: usize, _a1: usize, _a2: u32) -> i32 { hit(1372); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1373(_a0: usize) -> usize { hit(1373); crate::trace::object(1373) }
unsafe extern "C" fn s1374(_a0: usize) -> i32 { hit(1374); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1375(_a0: usize) -> i32 { hit(1375); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1376(_a0: usize) -> usize { hit(1376); crate::trace::object(1376) }
unsafe extern "C" fn s1377() -> usize { hit(1377); crate::trace::object(1377) }
unsafe extern "C" fn s1378(_a0: usize) { hit(1378);  }
unsafe extern "C" fn s1379(_a0: usize, _a1: usize, _a2: i32) { hit(1379);  }
unsafe extern "C" fn s1380(_a0: usize, _a1: usize, _a2: i32) -> i32 { hit(1380); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1381(_a0: usize) -> usize { hit(1381); crate::trace::object(1381) }
unsafe extern "C" fn s1382(_a0: usize) -> i32 { hit(1382); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1383(_a0: usize) -> u32 { hit(1383); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1384(_a0: usize, _a1: u32) { hit(1384);  }
unsafe extern "C" fn s1385(_a0: usize) -> u32 { hit(1385); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1386(_a0: usize, _a1: u32) { hit(1386);  }
unsafe extern "C" fn s1387(_a0: usize) -> u32 { hit(1387); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1388(_a0: usize, _a1: u32) { hit(1388);  }
unsafe extern "C" fn s1389(_a0: usize) -> u32 { hit(1389); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1390(_a0: usize, _a1: u32) { hit(1390);  }
unsafe extern "C" fn s1391(_a0: usize) -> u32 { hit(1391); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1392(_a0: usize, _a1: u32) { hit(1392);  }
unsafe extern "C" fn s1393(_a0: usize) -> f32 { hit(1393); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1394(_a0: usize, _a1: f32) { hit(1394);  }
unsafe extern "C" fn s1395(_a0: usize) { hit(1395);  }
unsafe extern "C" fn s1396(_a0: usize) -> i32 { hit(1396); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1397(_a0: usize) -> i32 { hit(1397); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1398(_a0: usize, _a1: u64) -> i32 { hit(1398); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1399(_a0: usize) -> u32 { hit(1399); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1400(_a0: usize) -> i32 { hit(1400); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1401(_a0: usize, _a1: i32) -> usize { hit(1401); crate::trace::object(1401) }
unsafe extern "C" fn s1402(_a0: usize) -> u32 { hit(1402); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1403(_a0: usize, _a1: usize, _a2: u32) -> i32 { hit(1403); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1404(_a0: usize) -> u64 { hit(1404); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1405(_a0: usize) -> u32 { hit(1405); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1406(_a0: usize) { hit(1406);  }
unsafe extern "C" fn s1407(_a0: usize) -> i32 { hit(1407); 1 }
unsafe extern "C" fn s1408(_a0: usize) -> i32 { hit(1408); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1409(_a0: usize) -> i32 { hit(1409); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1410(_a0: usize) -> usize { hit(1410); crate::trace::object(1410) }
unsafe extern "C" fn s1411(_a0: usize) -> usize { hit(1411); crate::trace::object(1411) }
unsafe extern "C" fn s1412(_a0: usize) -> S_System_Guid { hit(1412); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1413() -> usize { hit(1413); crate::trace::object(1413) }
unsafe extern "C" fn s1414(_a0: usize) { hit(1414);  }
unsafe extern "C" fn s1415(_a0: usize) -> usize { hit(1415); crate::trace::object(1415) }
unsafe extern "C" fn s1416(_a0: usize) -> i32 { hit(1416); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1417(_a0: usize, _a1: usize) { hit(1417);  }
unsafe extern "C" fn s1418(_a0: usize) { hit(1418);  }
unsafe extern "C" fn s1419(_a0: i32, _a1: i32) -> usize { hit(1419); crate::trace::object(1419) }
unsafe extern "C" fn s1420(_a0: usize) -> i32 { hit(1420); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1421(_a0: usize, _a1: i32) { hit(1421);  }
unsafe extern "C" fn s1422(_a0: usize, _a1: i32) -> usize { hit(1422); crate::trace::object(1422) }
unsafe extern "C" fn s1423(_a0: usize) { hit(1423);  }
unsafe extern "C" fn s1424(_a0: i32, _a1: i32) -> usize { hit(1424); crate::trace::object(1424) }
unsafe extern "C" fn s1425(_a0: usize) -> i32 { hit(1425); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1426(_a0: usize, _a1: i32) -> usize { hit(1426); crate::trace::object(1426) }
unsafe extern "C" fn s1427(_a0: usize) { hit(1427);  }
unsafe extern "C" fn s1428(_a0: i32, _a1: i32) -> usize { hit(1428); crate::trace::object(1428) }
unsafe extern "C" fn s1429(_a0: usize) -> i32 { hit(1429); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1430(_a0: usize, _a1: i32) -> S_MeshTraceOutput { hit(1430); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1431(_a0: usize) { hit(1431);  }
unsafe extern "C" fn s1432(_a0: i32, _a1: i32) -> usize { hit(1432); crate::trace::object(1432) }
unsafe extern "C" fn s1433(_a0: usize) -> i32 { hit(1433); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1434(_a0: usize, _a1: i32) -> S_PhysicsTrace_Result { hit(1434); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1435(_a0: usize) { hit(1435);  }
unsafe extern "C" fn s1436(_a0: usize) { hit(1436);  }
unsafe extern "C" fn s1437(_a0: i32, _a1: i32) -> usize { hit(1437); crate::trace::object(1437) }
unsafe extern "C" fn s1438(_a0: usize) -> i32 { hit(1438); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1439(_a0: usize, _a1: i32) { hit(1439);  }
unsafe extern "C" fn s1440(_a0: usize, _a1: i32) -> S_Vector3 { hit(1440); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1441(_a0: usize) { hit(1441);  }
unsafe extern "C" fn s1442(_a0: i32, _a1: i32) -> usize { hit(1442); crate::trace::object(1442) }
unsafe extern "C" fn s1443(_a0: usize) -> i32 { hit(1443); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1444(_a0: usize, _a1: i32) { hit(1444);  }
unsafe extern "C" fn s1445(_a0: usize, _a1: i32) -> f32 { hit(1445); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1446(_a0: usize) { hit(1446);  }
unsafe extern "C" fn s1447(_a0: i32, _a1: i32) -> usize { hit(1447); crate::trace::object(1447) }
unsafe extern "C" fn s1448(_a0: usize) -> i32 { hit(1448); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1449(_a0: usize, _a1: i32) { hit(1449);  }
unsafe extern "C" fn s1450(_a0: usize, _a1: i32) -> u32 { hit(1450); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1451(_a0: usize) { hit(1451);  }
unsafe extern "C" fn s1452(_a0: usize) -> i32 { hit(1452); 1 }
unsafe extern "C" fn s1453(_a0: usize) -> i32 { hit(1453); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1454(_a0: usize) -> i32 { hit(1454); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1455(_a0: usize) -> usize { hit(1455); crate::trace::object(1455) }
unsafe extern "C" fn s1456(_a0: usize) -> usize { hit(1456); crate::trace::object(1456) }
unsafe extern "C" fn s1457(_a0: usize) -> S_System_Guid { hit(1457); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1458(_a0: usize) -> usize { hit(1458); crate::trace::object(1458) }
unsafe extern "C" fn s1459(_a0: usize, _a1: usize, _a2: i32) -> usize { hit(1459); crate::trace::object(1459) }
unsafe extern "C" fn s1460(_a0: usize) -> usize { hit(1460); crate::trace::object(1460) }
unsafe extern "C" fn s1461(_a0: usize, _a1: usize, _a2: i64, _a3: u32, _a4: i32) -> i32 { hit(1461); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1462(_a0: usize, _a1: usize, _a2: i64, _a3: u32) -> i32 { hit(1462); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1463(_a0: usize, _a1: i64) -> usize { hit(1463); crate::trace::object(1463) }
unsafe extern "C" fn s1464(_a0: usize, _a1: i64) -> usize { hit(1464); crate::trace::object(1464) }
unsafe extern "C" fn s1465(_a0: usize) -> usize { hit(1465); crate::trace::object(1465) }
unsafe extern "C" fn s1466(_a0: usize, _a1: i64) -> i32 { hit(1466); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1467(_a0: usize) -> i32 { hit(1467); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1468(_a0: usize) -> i32 { hit(1468); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1469(_a0: usize, _a1: i64, _a2: usize, _a3: usize) -> i32 { hit(1469); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1470(_a0: usize, _a1: i64, _a2: u64, _a3: u64, _a4: usize) -> i32 { hit(1470); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1471(_a0: usize) -> usize { hit(1471); crate::trace::object(1471) }
unsafe extern "C" fn s1472(_a0: usize, _a1: i64, _a2: u64, _a3: u64) -> usize { hit(1472); crate::trace::object(1472) }
unsafe extern "C" fn s1473(_a0: usize, _a1: usize, _a2: usize, _a3: i32) -> i32 { hit(1473); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1474() -> usize { hit(1474); crate::trace::object(1474) }
unsafe extern "C" fn s1475(_a0: usize) { hit(1475);  }
unsafe extern "C" fn s1476(_a0: usize, _a1: u64) { hit(1476);  }
unsafe extern "C" fn s1477(_a0: usize, _a1: usize) { hit(1477);  }
unsafe extern "C" fn s1478(_a0: usize) { hit(1478);  }
unsafe extern "C" fn s1479(_a0: usize) -> usize { hit(1479); crate::trace::object(1479) }
unsafe extern "C" fn s1480(_a0: usize) -> usize { hit(1480); crate::trace::object(1480) }
unsafe extern "C" fn s1481(_a0: usize) -> u8 { hit(1481); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1482(_a0: usize, _a1: u8) { hit(1482);  }
unsafe extern "C" fn s1483(_a0: usize) -> u8 { hit(1483); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1484(_a0: usize, _a1: u8) { hit(1484);  }
unsafe extern "C" fn s1485(_a0: usize) { hit(1485);  }
unsafe extern "C" fn s1486(_a0: usize) -> u64 { hit(1486); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1487(_a0: usize, _a1: u64) -> u64 { hit(1487); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1488(_a0: usize) -> u64 { hit(1488); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1489(_a0: usize) -> u64 { hit(1489); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1490(_a0: usize, _a1: u64) -> u64 { hit(1490); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1491(_a0: usize) -> u64 { hit(1491); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1492(_a0: usize) -> u64 { hit(1492); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1493(_a0: usize) -> i32 { hit(1493); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1494(_a0: usize, _a1: i32) { hit(1494);  }
unsafe extern "C" fn s1495(_a0: u32) -> usize { hit(1495); crate::trace::object(1495) }
unsafe extern "C" fn s1496(_a0: usize) { hit(1496);  }
unsafe extern "C" fn s1497(_a0: usize, _a1: usize, _a2: usize) -> i32 { hit(1497); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1498(_a0: usize) { hit(1498);  }
unsafe extern "C" fn s1499(_a0: usize) { hit(1499);  }
unsafe extern "C" fn s1500(_a0: usize) { hit(1500);  }
unsafe extern "C" fn s1501(_a0: usize, _a1: f64) { hit(1501);  }
unsafe extern "C" fn s1502(_a0: usize) -> i32 { hit(1502); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1503(_a0: usize, _a1: i32) { hit(1503);  }
unsafe extern "C" fn s1504(_a0: usize) -> f32 { hit(1504); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1505(_a0: usize) -> f32 { hit(1505); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1506(_a0: usize) -> i32 { hit(1506); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1507(_a0: usize) { hit(1507);  }
unsafe extern "C" fn s1508(_a0: usize) -> i32 { hit(1508); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1509(_a0: usize) -> i32 { hit(1509); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1510(_a0: usize, _a1: i32) { hit(1510);  }
unsafe extern "C" fn s1511(_a0: usize) -> i32 { hit(1511); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1512(_a0: usize) -> i32 { hit(1512); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1513(_a0: usize) { hit(1513);  }
unsafe extern "C" fn s1514(_a0: usize, _a1: usize) -> usize { hit(1514); crate::trace::object(1514) }
unsafe extern "C" fn s1515(_a0: usize, _a1: usize) { hit(1515);  }
unsafe extern "C" fn s1516(_a0: usize) -> f32 { hit(1516); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1517(_a0: usize) -> f64 { hit(1517); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1518(_a0: usize) -> f64 { hit(1518); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1519(_a0: usize) -> i32 { hit(1519); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1520(_a0: usize) -> i32 { hit(1520); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1521(_a0: usize) -> i32 { hit(1521); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1522(_a0: usize) -> usize { hit(1522); crate::trace::object(1522) }
unsafe extern "C" fn s1523(_a0: usize) -> i32 { hit(1523); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1524() -> usize { hit(1524); crate::trace::object(1524) }
unsafe extern "C" fn s1525(_a0: usize) { hit(1525);  }
unsafe extern "C" fn s1526(_a0: usize) { hit(1526);  }
unsafe extern "C" fn s1527(_a0: usize) -> i32 { hit(1527); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1528(_a0: usize, _a1: usize, _a2: i64) { hit(1528);  }
unsafe extern "C" fn s1529(_a0: usize, _a1: usize) { hit(1529);  }
unsafe extern "C" fn s1530(_a0: usize, _a1: usize, _a2: i32, _a3: i32, _a4: i32, _a5: i32, _a6: i32, _a7: i32, _a8: usize, _a9: usize, _a10: i32, _a11: usize, _a12: i32) -> i32 { hit(1530); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1531(_a0: usize) -> usize { hit(1531); crate::trace::object(1531) }
unsafe extern "C" fn s1532(_a0: usize) -> usize { hit(1532); crate::trace::object(1532) }
unsafe extern "C" fn s1533(_a0: usize) -> i64 { hit(1533); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1534(_a0: usize, _a1: usize) { hit(1534);  }
unsafe extern "C" fn s1535(_a0: usize) -> usize { hit(1535); crate::trace::object(1535) }
unsafe extern "C" fn s1536(_a0: usize) -> usize { hit(1536); crate::trace::object(1536) }
unsafe extern "C" fn s1537(_a0: usize, _a1: usize) { hit(1537);  }
unsafe extern "C" fn s1538(_a0: usize, _a1: f32) { hit(1538);  }
unsafe extern "C" fn s1539(_a0: usize, _a1: i32) { hit(1539);  }
unsafe extern "C" fn s1540(_a0: usize) -> usize { hit(1540); crate::trace::object(1540) }
unsafe extern "C" fn s1541(_a0: usize) { hit(1541);  }
unsafe extern "C" fn s1542(_a0: usize) -> i32 { hit(1542); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1543(_a0: usize) -> i32 { hit(1543); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1544(_a0: usize) -> f32 { hit(1544); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1545(_a0: usize) -> f32 { hit(1545); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1546(_a0: usize) -> usize { hit(1546); crate::trace::object(1546) }
unsafe extern "C" fn s1547(_a0: usize) -> i64 { hit(1547); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1548(_a0: usize) { hit(1548);  }
unsafe extern "C" fn s1549(_a0: usize, _a1: usize, _a2: usize, _a3: i32) { hit(1549);  }
unsafe extern "C" fn s1550(_a0: usize) -> usize { hit(1550); crate::trace::object(1550) }
unsafe extern "C" fn s1551(_a0: usize) { hit(1551);  }
unsafe extern "C" fn s1552(_a0: usize, _a1: i32, _a2: usize, _a3: u32) { hit(1552);  }
unsafe extern "C" fn s1553(_a0: usize) { hit(1553);  }
unsafe extern "C" fn s1554(_a0: usize, _a1: i32) -> usize { hit(1554); crate::trace::object(1554) }
unsafe extern "C" fn s1555(_a0: usize) -> usize { hit(1555); crate::trace::object(1555) }
unsafe extern "C" fn s1556(_a0: usize) -> usize { hit(1556); crate::trace::object(1556) }
unsafe extern "C" fn s1557(_a0: usize) -> usize { hit(1557); crate::trace::object(1557) }
unsafe extern "C" fn s1558(_a0: usize) -> usize { hit(1558); crate::trace::object(1558) }
unsafe extern "C" fn s1559(_a0: usize) -> u32 { hit(1559); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1560(_a0: u32) -> usize { hit(1560); crate::trace::object(1560) }
unsafe extern "C" fn s1561(_a0: usize, _a1: usize, _a2: i32) { hit(1561);  }
unsafe extern "C" fn s1562(_a0: usize, _a1: usize) -> i32 { hit(1562); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1563() -> u64 { hit(1563); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1564(_a0: usize, _a1: i32) -> usize { hit(1564); unsafe { crate::impls::EngineGlue_ReadCompiledResourceFileJson(_a0, _a1) } }
unsafe extern "C" fn s1565(_a0: usize, _a1: usize, _a2: i32, _a3: usize) -> usize { hit(1565); unsafe { crate::impls::EngineGlue_ReadCompiledResourceFileBlock(_a0, _a1, _a2, _a3) } }
unsafe extern "C" fn s1566(_a0: usize) -> usize { hit(1566); crate::trace::object(1566) }
unsafe extern "C" fn s1567(_a0: i32) { hit(1567);  }
unsafe extern "C" fn s1568() { hit(1568);  }
unsafe extern "C" fn s1569() { hit(1569);  }
unsafe extern "C" fn s1570() -> usize { hit(1570); crate::trace::object(1570) }
unsafe extern "C" fn s1571() { hit(1571);  }
unsafe extern "C" fn s1572(_a0: usize, _a1: usize) { hit(1572);  }
unsafe extern "C" fn s1573(_a0: i32, _a1: usize, _a2: usize) { hit(1573);  }
unsafe extern "C" fn s1574() { hit(1574);  }
unsafe extern "C" fn s1575() -> usize { hit(1575); crate::trace::object(1575) }
unsafe extern "C" fn s1576(_a0: i32, _a1: i32) -> usize { hit(1576); crate::trace::object(1576) }
unsafe extern "C" fn s1577(_a0: usize) { hit(1577);  }
unsafe extern "C" fn s1578(_a0: usize, _a1: usize, _a2: i64) -> i32 { hit(1578); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1579(_a0: usize, _a1: usize) -> i32 { hit(1579); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1580(_a0: usize, _a1: usize, _a2: i64) -> i32 { hit(1580); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1581(_a0: usize, _a1: usize, _a2: i64) -> i32 { hit(1581); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1582(_a0: usize, _a1: usize, _a2: i64) -> i32 { hit(1582); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1583(_a0: usize, _a1: usize, _a2: i64) -> i32 { hit(1583); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1584(_a0: usize, _a1: usize) -> i32 { hit(1584); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1585(_a0: usize, _a1: usize, _a2: i32, _a3: i64, _a4: i64) { hit(1585);  }
unsafe extern "C" fn s1586(_a0: usize, _a1: i32, _a2: i32, _a3: i32) { hit(1586);  }
unsafe extern "C" fn s1587(_a0: usize) { hit(1587);  }
unsafe extern "C" fn s1588(_a0: usize, _a1: i32, _a2: f32) { hit(1588);  }
unsafe extern "C" fn s1589(_a0: usize) { hit(1589);  }
unsafe extern "C" fn s1590(_a0: usize) { hit(1590);  }
unsafe extern "C" fn s1591(_a0: usize) { hit(1591);  }
unsafe extern "C" fn s1592(_a0: usize) { hit(1592);  }
unsafe extern "C" fn s1593(_a0: usize) { hit(1593);  }
unsafe extern "C" fn s1594(_a0: usize, _a1: usize) -> i32 { hit(1594); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1595(_a0: usize, _a1: usize) -> i32 { hit(1595); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1596(_a0: usize, _a1: usize, _a2: i32) -> i32 { hit(1596); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1597(_a0: usize, _a1: usize, _a2: i32) -> i32 { hit(1597); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1598(_a0: usize, _a1: usize, _a2: i32) -> i32 { hit(1598); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1599(_a0: usize, _a1: usize, _a2: i32) -> i32 { hit(1599); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1600(_a0: usize, _a1: i32, _a2: i32, _a3: i32, _a4: i32) -> f32 { hit(1600); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1601(_a0: usize, _a1: i32, _a2: i32, _a3: i32, _a4: i32) -> f32 { hit(1601); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1602(_a0: usize, _a1: i32, _a2: i32, _a3: i32, _a4: i32) -> f32 { hit(1602); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1603(_a0: usize, _a1: i32, _a2: i32, _a3: i32) -> f32 { hit(1603); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1604(_a0: usize, _a1: i32, _a2: i32, _a3: i32) -> S_Vector3 { hit(1604); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1605(_a0: usize) -> i32 { hit(1605); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1606(_a0: usize) -> i32 { hit(1606); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1607(_a0: usize) -> i32 { hit(1607); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1608(_a0: usize, _a1: i32, _a2: i32, _a3: i32) { hit(1608);  }
unsafe extern "C" fn s1609(_a0: usize, _a1: usize, _a2: i32, _a3: i64, _a4: i32, _a5: i32, _a6: u32) -> i32 { hit(1609); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1610(_a0: usize) -> usize { hit(1610); crate::trace::object(1610) }
unsafe extern "C" fn s1611(_a0: usize, _a1: usize) { hit(1611);  }
unsafe extern "C" fn s1612(_a0: usize) { hit(1612);  }
unsafe extern "C" fn s1613(_a0: usize, _a1: usize) { hit(1613);  }
unsafe extern "C" fn s1614() { hit(1614); unsafe { crate::impls::Glue_Networking_RunCallbacks() } }
unsafe extern "C" fn s1615(_a0: i32, _a1: usize) { hit(1615);  }
unsafe extern "C" fn s1616(_a0: usize) -> i64 { hit(1616); unsafe { crate::impls::Glue_Networking_GetAuthenticationStatus(_a0) } }
unsafe extern "C" fn s1617(_a0: usize) -> i64 { hit(1617); unsafe { crate::impls::Glue_Networking_GetRelayNetworkStatus(_a0) } }
unsafe extern "C" fn s1618(_a0: i32) -> S_Sandbox_Network_HSteamListenSocket { hit(1618); unsafe { crate::impls::Glue_Networking_CreateSocket(_a0) } }
unsafe extern "C" fn s1619(_a0: i32, _a1: i32) -> S_Sandbox_Network_HSteamListenSocket { hit(1619); unsafe { crate::impls::Glue_Networking_CreateIpBasedSocket(_a0, _a1) } }
unsafe extern "C" fn s1620(_a0: S_Sandbox_Network_HSteamListenSocket) { hit(1620); unsafe { crate::impls::Glue_Networking_CloseSocket(_a0) } }
unsafe extern "C" fn s1621(_a0: S_Sandbox_Network_HSteamListenSocket) -> usize { hit(1621); unsafe { crate::impls::Glue_Networking_GetSocketAddress(_a0) } }
unsafe extern "C" fn s1622() { hit(1622);  }
unsafe extern "C" fn s1623() -> usize { hit(1623); unsafe { crate::impls::Glue_Networking_GetIdentity() } }
unsafe extern "C" fn s1624() -> S_Sandbox_Network_HSteamNetPollGroup { hit(1624); unsafe { crate::impls::Glue_Networking_CreatePollGroup() } }
unsafe extern "C" fn s1625(_a0: S_Sandbox_Network_HSteamNetPollGroup) { hit(1625); unsafe { crate::impls::Glue_Networking_DestroyPollGroup(_a0) } }
unsafe extern "C" fn s1626(_a0: S_Sandbox_Network_HSteamNetConnection, _a1: S_Sandbox_Network_HSteamNetPollGroup) { hit(1626); unsafe { crate::impls::Glue_Networking_SetPollGroup(_a0, _a1) } }
unsafe extern "C" fn s1627(_a0: S_Sandbox_Network_HSteamNetPollGroup, _a1: usize, _a2: i32) -> i32 { hit(1627); unsafe { crate::impls::Glue_Networking_GetPollGroupMessages(_a0, _a1, _a2) } }
unsafe extern "C" fn s1628(_a0: u64, _a1: i32) -> S_Sandbox_Network_HSteamNetConnection { hit(1628); unsafe { crate::impls::Glue_Networking_ConnectToSteamId(_a0, _a1) } }
unsafe extern "C" fn s1629(_a0: usize) -> S_Sandbox_Network_HSteamNetConnection { hit(1629); unsafe { crate::impls::Glue_Networking_ConnectToIpAddress(_a0) } }
unsafe extern "C" fn s1630(_a0: S_Sandbox_Network_HSteamNetConnection, _a1: i32, _a2: usize) { hit(1630); unsafe { crate::impls::Glue_Networking_CloseConnection(_a0, _a1, _a2) } }
unsafe extern "C" fn s1631(_a0: S_Sandbox_Network_HSteamNetConnection) { hit(1631);  }
unsafe extern "C" fn s1632(_a0: usize) -> usize { hit(1632); unsafe { crate::impls::Glue_Networking_GetMessageDataBuffer(_a0) } }
unsafe extern "C" fn s1633(_a0: S_Sandbox_Network_HSteamNetConnection, _a1: usize, _a2: i32, _a3: i32) -> usize { hit(1633); crate::trace::object(1633) }
unsafe extern "C" fn s1634(_a0: S_Sandbox_Network_HSteamNetConnection, _a1: i32, _a2: i32) -> usize { hit(1634); unsafe { crate::impls::Glue_Networking_AllocateMessage(_a0, _a1, _a2) } }
unsafe extern "C" fn s1635(_a0: S_Sandbox_Network_HSteamNetConnection) { hit(1635);  }
unsafe extern "C" fn s1636(_a0: S_Sandbox_Network_HSteamNetConnection, _a1: usize, _a2: i32, _a3: i32) -> i64 { hit(1636); unsafe { crate::impls::Glue_Networking_SendMessage(_a0, _a1, _a2, _a3) } }
unsafe extern "C" fn s1637(_a0: usize, _a1: i32) { hit(1637); unsafe { crate::impls::Glue_Networking_SendMessages(_a0, _a1) } }
unsafe extern "C" fn s1638(_a0: S_Sandbox_Network_HSteamNetConnection, _a1: usize, _a2: i32) -> i32 { hit(1638); unsafe { crate::impls::Glue_Networking_GetConnectionMessages(_a0, _a1, _a2) } }
unsafe extern "C" fn s1639(_a0: S_Sandbox_Network_HSteamNetConnection) -> i32 { hit(1639); unsafe { crate::impls::Glue_Networking_GetConnectionState(_a0) } }
unsafe extern "C" fn s1640(_a0: S_Sandbox_Network_HSteamNetConnection) -> usize { hit(1640); unsafe { crate::impls::Glue_Networking_GetConnectionDescription(_a0) } }
unsafe extern "C" fn s1641(_a0: S_Sandbox_Network_HSteamNetConnection) -> u64 { hit(1641); unsafe { crate::impls::Glue_Networking_GetConnectionSteamId(_a0) } }
unsafe extern "C" fn s1642(_a0: usize) { hit(1642);  }
unsafe extern "C" fn s1643(_a0: i64) -> i64 { hit(1643); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1644(_a0: i32) -> i32 { hit(1644); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1645() -> i32 { hit(1645); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1646(_a0: usize, _a1: S_System_Guid) -> usize { hit(1646); crate::trace::object(1646) }
unsafe extern "C" fn s1647(_a0: usize, _a1: S_System_Guid) -> usize { hit(1647); crate::trace::object(1647) }
unsafe extern "C" fn s1648() -> i32 { hit(1648); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1649() -> i32 { hit(1649); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1650(_a0: usize, _a1: usize, _a2: i32, _a3: i32) -> i32 { hit(1650); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1651() -> i32 { hit(1651); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1652(_a0: i32, _a1: i32) { hit(1652);  }
unsafe extern "C" fn s1653(_a0: usize, _a1: S_System_Guid) -> usize { hit(1653); crate::trace::object(1653) }
unsafe extern "C" fn s1654(_a0: usize) -> usize { hit(1654); crate::trace::object(1654) }
unsafe extern "C" fn s1655(_a0: usize, _a1: S_System_Guid) -> usize { hit(1655); crate::trace::object(1655) }
unsafe extern "C" fn s1656(_a0: usize, _a1: S_System_Guid) -> usize { hit(1656); crate::trace::object(1656) }
unsafe extern "C" fn s1657(_a0: usize, _a1: S_System_Guid) -> usize { hit(1657); crate::trace::object(1657) }
unsafe extern "C" fn s1658(_a0: usize) -> usize { hit(1658); crate::trace::object(1658) }
unsafe extern "C" fn s1659(_a0: usize) -> i64 { hit(1659); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1660(_a0: usize) -> S_NativeEngine_AnimVariant { hit(1660); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1661(_a0: usize) -> S_NativeEngine_AnimVariant { hit(1661); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1662(_a0: usize) -> S_NativeEngine_AnimVariant { hit(1662); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1663(_a0: usize) -> i32 { hit(1663); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1664(_a0: usize, _a1: i32) -> usize { hit(1664); crate::trace::object(1664) }
unsafe extern "C" fn s1665(_a0: usize) -> usize { hit(1665); crate::trace::object(1665) }
unsafe extern "C" fn s1666(_a0: usize) -> S_NativeEngine_AnimVariant { hit(1666); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1667(_a0: usize) -> i64 { hit(1667); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1668(_a0: usize) -> i32 { hit(1668); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1669(_a0: usize, _a1: usize) { hit(1669);  }
unsafe extern "C" fn s1670(_a0: usize) -> i32 { hit(1670); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1671(_a0: usize, _a1: i32) -> usize { hit(1671); crate::trace::object(1671) }
unsafe extern "C" fn s1672(_a0: usize, _a1: usize) -> usize { hit(1672); crate::trace::object(1672) }
unsafe extern "C" fn s1673(_a0: usize) { hit(1673);  }
unsafe extern "C" fn s1674(_a0: usize) -> i32 { hit(1674); 1 }
unsafe extern "C" fn s1675(_a0: usize) -> i32 { hit(1675); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1676(_a0: usize) -> i32 { hit(1676); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1677(_a0: usize) -> usize { hit(1677); crate::trace::object(1677) }
unsafe extern "C" fn s1678(_a0: usize) -> usize { hit(1678); crate::trace::object(1678) }
unsafe extern "C" fn s1679(_a0: usize) -> S_System_Guid { hit(1679); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1680(_a0: usize) -> usize { hit(1680); crate::trace::object(1680) }
unsafe extern "C" fn s1681(_a0: usize) -> usize { hit(1681); crate::trace::object(1681) }
unsafe extern "C" fn s1682(_a0: usize) -> usize { hit(1682); crate::trace::object(1682) }
unsafe extern "C" fn s1683(_a0: usize) -> usize { hit(1683); crate::trace::object(1683) }
unsafe extern "C" fn s1684(_a0: usize) -> i64 { hit(1684); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1685(_a0: usize) { hit(1685);  }
unsafe extern "C" fn s1686(_a0: usize) -> i32 { hit(1686); 1 }
unsafe extern "C" fn s1687(_a0: usize) -> i32 { hit(1687); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1688(_a0: usize) -> i32 { hit(1688); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1689(_a0: usize) -> usize { hit(1689); crate::trace::object(1689) }
unsafe extern "C" fn s1690(_a0: usize) -> usize { hit(1690); crate::trace::object(1690) }
unsafe extern "C" fn s1691(_a0: usize) -> S_System_Guid { hit(1691); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1692(_a0: usize) -> usize { hit(1692); crate::trace::object(1692) }
unsafe extern "C" fn s1693(_a0: usize) -> usize { hit(1693); crate::trace::object(1693) }
unsafe extern "C" fn s1694(_a0: usize) -> u64 { hit(1694); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1695(_a0: usize) -> i32 { hit(1695); 1 }
unsafe extern "C" fn s1696(_a0: usize, _a1: S_Sandbox_StringToken) -> usize { hit(1696); crate::trace::object(1696) }
unsafe extern "C" fn s1697(_a0: usize) -> usize { hit(1697); crate::trace::object(1697) }
unsafe extern "C" fn s1698(_a0: usize, _a1: usize) -> usize { hit(1698); crate::trace::object(1698) }
unsafe extern "C" fn s1699(_a0: usize) -> usize { hit(1699); crate::trace::object(1699) }
unsafe extern "C" fn s1700(_a0: usize) { hit(1700);  }
unsafe extern "C" fn s1701(_a0: usize) -> usize { hit(1701); crate::trace::object(1701) }
unsafe extern "C" fn s1702(_a0: usize, _a1: S_Sandbox_StringToken, _a2: i32) -> i32 { hit(1702); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1703(_a0: usize, _a1: S_Sandbox_StringToken, _a2: i32) -> i32 { hit(1703); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1704(_a0: usize, _a1: S_Sandbox_StringToken, _a2: f32) -> f32 { hit(1704); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1705(_a0: usize, _a1: S_Sandbox_StringToken) -> usize { hit(1705); crate::trace::object(1705) }
unsafe extern "C" fn s1706(_a0: usize, _a1: usize) -> i32 { hit(1706); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1707(_a0: usize, _a1: usize, _a2: usize) { hit(1707);  }
unsafe extern "C" fn s1708(_a0: usize, _a1: usize, _a2: usize) -> usize { hit(1708); crate::trace::object(1708) }
unsafe extern "C" fn s1709(_a0: usize, _a1: usize, _a2: usize) { hit(1709);  }
unsafe extern "C" fn s1710(_a0: usize, _a1: usize) -> S_Vector4 { hit(1710); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1711(_a0: usize, _a1: usize, _a2: usize) { hit(1711);  }
unsafe extern "C" fn s1712(_a0: usize, _a1: usize) -> usize { hit(1712); crate::trace::object(1712) }
unsafe extern "C" fn s1713(_a0: usize, _a1: i32) { hit(1713);  }
unsafe extern "C" fn s1714(_a0: usize) -> i32 { hit(1714); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1715(_a0: usize) { hit(1715);  }
unsafe extern "C" fn s1716(_a0: usize) -> i32 { hit(1716); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1717(_a0: usize) -> i32 { hit(1717); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1718(_a0: usize, _a1: i32, _a2: usize, _a3: usize) -> i32 { hit(1718); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1719(_a0: usize, _a1: i32, _a2: usize, _a3: usize, _a4: usize) -> i32 { hit(1719); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1720(_a0: usize, _a1: usize) -> i32 { hit(1720); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1721(_a0: usize, _a1: usize) -> i32 { hit(1721); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1722(_a0: usize) { hit(1722);  }
unsafe extern "C" fn s1723(_a0: usize) { hit(1723);  }
unsafe extern "C" fn s1724(_a0: usize) -> i32 { hit(1724); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1725(_a0: usize, _a1: usize) { hit(1725);  }
unsafe extern "C" fn s1726(_a0: usize, _a1: usize) { hit(1726);  }
unsafe extern "C" fn s1727(_a0: usize, _a1: usize) { hit(1727);  }
unsafe extern "C" fn s1728(_a0: usize, _a1: usize) { hit(1728);  }
unsafe extern "C" fn s1729(_a0: usize) -> i32 { hit(1729); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1730(_a0: usize, _a1: i32) -> i32 { hit(1730); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1731(_a0: usize, _a1: i32) -> usize { hit(1731); crate::trace::object(1731) }
unsafe extern "C" fn s1732(_a0: usize, _a1: i32) -> u32 { hit(1732); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1733(_a0: usize, _a1: i32) -> i32 { hit(1733); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1734(_a0: usize, _a1: usize) -> i32 { hit(1734); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1735(_a0: usize, _a1: usize) -> i32 { hit(1735); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1736(_a0: usize) -> i32 { hit(1736); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1737(_a0: usize, _a1: i32) -> i32 { hit(1737); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1738(_a0: usize, _a1: usize) { hit(1738);  }
unsafe extern "C" fn s1739(_a0: usize) -> S_Vector3 { hit(1739); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1740(_a0: usize) -> S_Vector3 { hit(1740); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1741(_a0: usize, _a1: S_Sandbox_StringToken) { hit(1741);  }
unsafe extern "C" fn s1742(_a0: usize) -> f32 { hit(1742); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1743(_a0: usize, _a1: f32) { hit(1743);  }
unsafe extern "C" fn s1744(_a0: usize, _a1: f32) { hit(1744);  }
unsafe extern "C" fn s1745(_a0: usize, _a1: f32) { hit(1745);  }
unsafe extern "C" fn s1746(_a0: usize) -> i32 { hit(1746); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1747(_a0: usize) -> i32 { hit(1747); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1748(_a0: usize, _a1: i32) -> usize { hit(1748); crate::trace::object(1748) }
unsafe extern "C" fn s1749(_a0: usize, _a1: usize, _a2: usize, _a3: usize) -> usize { hit(1749); crate::trace::object(1749) }
unsafe extern "C" fn s1750(_a0: usize) -> i32 { hit(1750); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1751(_a0: usize, _a1: f32) { hit(1751);  }
unsafe extern "C" fn s1752(_a0: usize) -> f32 { hit(1752); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1753(_a0: usize) -> i32 { hit(1753); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1754(_a0: usize, _a1: i32) { hit(1754);  }
unsafe extern "C" fn s1755(_a0: usize, _a1: f32) { hit(1755);  }
unsafe extern "C" fn s1756(_a0: usize) -> f32 { hit(1756); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1757(_a0: usize) -> S_Vector3 { hit(1757); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1758(_a0: usize) -> S_Vector3 { hit(1758); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1759(_a0: usize, _a1: usize) { hit(1759);  }
unsafe extern "C" fn s1760(_a0: usize, _a1: i32) { hit(1760);  }
unsafe extern "C" fn s1761(_a0: usize) -> i32 { hit(1761); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1762(_a0: usize, _a1: usize) { hit(1762);  }
unsafe extern "C" fn s1763(_a0: usize) -> S_Vector3 { hit(1763); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1764(_a0: usize, _a1: usize) { hit(1764);  }
unsafe extern "C" fn s1765(_a0: usize) -> S_Rotation { hit(1765); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1766(_a0: usize, _a1: usize, _a2: usize) { hit(1766);  }
unsafe extern "C" fn s1767(_a0: usize) -> S_Transform { hit(1767); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1768(_a0: usize, _a1: usize) { hit(1768);  }
unsafe extern "C" fn s1769(_a0: usize) -> S_Vector3 { hit(1769); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1770(_a0: usize, _a1: usize) -> S_Vector3 { hit(1770); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1771(_a0: usize, _a1: usize) { hit(1771);  }
unsafe extern "C" fn s1772(_a0: usize, _a1: usize) { hit(1772);  }
unsafe extern "C" fn s1773(_a0: usize) -> S_Vector3 { hit(1773); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1774(_a0: usize) { hit(1774);  }
unsafe extern "C" fn s1775(_a0: usize) { hit(1775);  }
unsafe extern "C" fn s1776(_a0: usize) -> i32 { hit(1776); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1777(_a0: usize) { hit(1777);  }
unsafe extern "C" fn s1778(_a0: usize) { hit(1778);  }
unsafe extern "C" fn s1779(_a0: usize, _a1: f32) { hit(1779);  }
unsafe extern "C" fn s1780(_a0: usize) -> f32 { hit(1780); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1781(_a0: usize) { hit(1781);  }
unsafe extern "C" fn s1782(_a0: usize) { hit(1782);  }
unsafe extern "C" fn s1783(_a0: usize) -> i32 { hit(1783); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1784(_a0: usize) -> i64 { hit(1784); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1785(_a0: usize, _a1: i64) { hit(1785);  }
unsafe extern "C" fn s1786(_a0: usize) -> i32 { hit(1786); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1787(_a0: usize, _a1: i32) -> i32 { hit(1787); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1788(_a0: usize, _a1: usize, _a2: f32, _a3: usize) -> i32 { hit(1788); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1789(_a0: usize, _a1: usize, _a2: usize, _a3: f32, _a4: usize) -> i32 { hit(1789); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1790(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize) -> i32 { hit(1790); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1791(_a0: usize, _a1: usize, _a2: usize, _a3: i32, _a4: usize, _a5: usize) -> i32 { hit(1791); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1792(_a0: usize, _a1: usize, _a2: usize, _a3: usize) -> i32 { hit(1792); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1793(_a0: usize, _a1: usize, _a2: usize, _a3: usize) -> i32 { hit(1793); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1794(_a0: usize, _a1: i32, _a2: usize, _a3: i32, _a4: usize, _a5: i32, _a6: usize) -> i32 { hit(1794); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1795(_a0: usize, _a1: usize, _a2: usize, _a3: i32, _a4: usize) -> i32 { hit(1795); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1796(_a0: usize, _a1: usize, _a2: usize, _a3: i32, _a4: i32, _a5: f32, _a6: f32, _a7: i32, _a8: usize) -> i32 { hit(1796); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1797(_a0: usize, _a1: usize) { hit(1797);  }
unsafe extern "C" fn s1798(_a0: usize) { hit(1798);  }
unsafe extern "C" fn s1799(_a0: usize, _a1: usize) { hit(1799);  }
unsafe extern "C" fn s1800(_a0: usize, _a1: usize, _a2: usize) { hit(1800);  }
unsafe extern "C" fn s1801(_a0: usize, _a1: usize) { hit(1801);  }
unsafe extern "C" fn s1802(_a0: usize, _a1: usize) { hit(1802);  }
unsafe extern "C" fn s1803(_a0: usize, _a1: usize, _a2: usize) { hit(1803);  }
unsafe extern "C" fn s1804(_a0: usize, _a1: usize) { hit(1804);  }
unsafe extern "C" fn s1805(_a0: usize) { hit(1805);  }
unsafe extern "C" fn s1806(_a0: usize) { hit(1806);  }
unsafe extern "C" fn s1807(_a0: usize) { hit(1807);  }
unsafe extern "C" fn s1808(_a0: usize) { hit(1808);  }
unsafe extern "C" fn s1809(_a0: usize) -> i32 { hit(1809); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1810(_a0: usize) { hit(1810);  }
unsafe extern "C" fn s1811(_a0: usize, _a1: i32) -> i32 { hit(1811); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1812(_a0: usize, _a1: f32) { hit(1812);  }
unsafe extern "C" fn s1813(_a0: usize) -> f32 { hit(1813); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1814(_a0: usize, _a1: f32) { hit(1814);  }
unsafe extern "C" fn s1815(_a0: usize) -> f32 { hit(1815); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1816(_a0: usize) -> S_BBox { hit(1816); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1817(_a0: usize) -> f32 { hit(1817); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1818(_a0: usize, _a1: usize) -> S_Vector3 { hit(1818); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1819(_a0: usize, _a1: S_Sandbox_StringToken) { hit(1819);  }
unsafe extern "C" fn s1820(_a0: usize) -> i32 { hit(1820); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1821(_a0: usize, _a1: usize, _a2: usize, _a3: f32) { hit(1821);  }
unsafe extern "C" fn s1822(_a0: usize, _a1: usize, _a2: usize) -> i32 { hit(1822); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1823(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize) -> i32 { hit(1823); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1824(_a0: usize, _a1: usize, _a2: i32) -> i32 { hit(1824); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1825(_a0: usize) -> S_Vector3 { hit(1825); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1826(_a0: usize) -> S_Rotation { hit(1826); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1827(_a0: usize, _a1: usize, _a2: usize) { hit(1827);  }
unsafe extern "C" fn s1828(_a0: usize) { hit(1828);  }
unsafe extern "C" fn s1829(_a0: usize) -> i32 { hit(1829); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1830(_a0: usize, _a1: i32, _a2: i32, _a3: i32, _a4: i32, _a5: i32, _a6: i32) { hit(1830);  }
unsafe extern "C" fn s1831(_a0: usize, _a1: usize, _a2: i32) -> i32 { hit(1831); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1832(_a0: usize, _a1: usize, _a2: i32) -> i32 { hit(1832); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1833(_a0: usize) { hit(1833);  }
unsafe extern "C" fn s1834(_a0: usize, _a1: i32) { hit(1834);  }
unsafe extern "C" fn s1835(_a0: usize, _a1: usize, _a2: usize, _a3: f32, _a4: f32, _a5: f32, _a6: usize, _a7: usize, _a8: f32) { hit(1835);  }
unsafe extern "C" fn s1836(_a0: usize, _a1: f32) { hit(1836);  }
unsafe extern "C" fn s1837(_a0: usize, _a1: i32) { hit(1837);  }
unsafe extern "C" fn s1838(_a0: usize) -> i32 { hit(1838); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1839(_a0: usize) -> i32 { hit(1839); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1840(_a0: usize) -> i32 { hit(1840); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1841(_a0: usize, _a1: usize, _a2: usize) { hit(1841);  }
unsafe extern "C" fn s1842(_a0: usize, _a1: usize, _a2: usize) { hit(1842);  }
unsafe extern "C" fn s1843(_a0: usize, _a1: usize, _a2: usize) { hit(1843);  }
unsafe extern "C" fn s1844(_a0: usize, _a1: usize, _a2: usize) { hit(1844);  }
unsafe extern "C" fn s1845(_a0: usize, _a1: i32) { hit(1845);  }
unsafe extern "C" fn s1846(_a0: usize) -> i32 { hit(1846); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1847(_a0: usize) -> i64 { hit(1847); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1848(_a0: usize, _a1: usize) { hit(1848);  }
unsafe extern "C" fn s1849(_a0: usize) -> S_Vector3 { hit(1849); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1850(_a0: usize, _a1: usize) { hit(1850);  }
unsafe extern "C" fn s1851(_a0: usize) -> S_Vector3 { hit(1851); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1852(_a0: usize, _a1: f32, _a2: f32) { hit(1852);  }
unsafe extern "C" fn s1853(_a0: usize, _a1: f32, _a2: f32) { hit(1853);  }
unsafe extern "C" fn s1854(_a0: usize, _a1: f32) { hit(1854);  }
unsafe extern "C" fn s1855(_a0: usize) -> f32 { hit(1855); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1856(_a0: usize, _a1: f32) { hit(1856);  }
unsafe extern "C" fn s1857(_a0: usize) -> f32 { hit(1857); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1858(_a0: usize, _a1: f32) { hit(1858);  }
unsafe extern "C" fn s1859(_a0: usize) -> f32 { hit(1859); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1860(_a0: usize, _a1: f32) { hit(1860);  }
unsafe extern "C" fn s1861(_a0: usize) -> f32 { hit(1861); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1862(_a0: usize, _a1: f32) { hit(1862);  }
unsafe extern "C" fn s1863(_a0: usize, _a1: f32) { hit(1863);  }
unsafe extern "C" fn s1864(_a0: usize, _a1: i32) { hit(1864);  }
unsafe extern "C" fn s1865(_a0: usize, _a1: f32, _a2: f32) { hit(1865);  }
unsafe extern "C" fn s1866(_a0: usize, _a1: i32) { hit(1866);  }
unsafe extern "C" fn s1867(_a0: usize, _a1: f32, _a2: f32) { hit(1867);  }
unsafe extern "C" fn s1868(_a0: usize, _a1: i32) { hit(1868);  }
unsafe extern "C" fn s1869(_a0: usize) -> f32 { hit(1869); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1870(_a0: usize) -> f32 { hit(1870); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1871(_a0: usize) -> f32 { hit(1871); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1872(_a0: usize) -> f32 { hit(1872); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1873(_a0: usize) -> f32 { hit(1873); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1874(_a0: usize, _a1: f32) { hit(1874);  }
unsafe extern "C" fn s1875(_a0: usize, _a1: f32) { hit(1875);  }
unsafe extern "C" fn s1876(_a0: usize, _a1: usize, _a2: f32) { hit(1876);  }
unsafe extern "C" fn s1877(_a0: usize, _a1: usize, _a2: f32, _a3: f32) { hit(1877);  }
unsafe extern "C" fn s1878(_a0: usize, _a1: usize) { hit(1878);  }
unsafe extern "C" fn s1879(_a0: usize, _a1: usize) { hit(1879);  }
unsafe extern "C" fn s1880(_a0: usize, _a1: f32) { hit(1880);  }
unsafe extern "C" fn s1881(_a0: usize, _a1: f32) { hit(1881);  }
unsafe extern "C" fn s1882(_a0: usize, _a1: f32) { hit(1882);  }
unsafe extern "C" fn s1883(_a0: usize, _a1: f32) { hit(1883);  }
unsafe extern "C" fn s1884(_a0: usize, _a1: f32) { hit(1884);  }
unsafe extern "C" fn s1885(_a0: usize, _a1: f32) { hit(1885);  }
unsafe extern "C" fn s1886(_a0: usize, _a1: f32) { hit(1886);  }
unsafe extern "C" fn s1887(_a0: usize, _a1: f32) { hit(1887);  }
unsafe extern "C" fn s1888(_a0: usize) -> S_Vector3 { hit(1888); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1889(_a0: usize) -> S_Vector3 { hit(1889); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1890(_a0: usize) -> f32 { hit(1890); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1891(_a0: usize) -> f32 { hit(1891); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1892(_a0: usize) -> f32 { hit(1892); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1893(_a0: usize) -> f32 { hit(1893); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1894(_a0: usize) -> f32 { hit(1894); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1895(_a0: usize) -> f32 { hit(1895); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1896(_a0: usize) -> f32 { hit(1896); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1897(_a0: usize) -> f32 { hit(1897); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1898(_a0: usize, _a1: i32) { hit(1898);  }
unsafe extern "C" fn s1899(_a0: usize) -> i32 { hit(1899); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1900(_a0: usize, _a1: f32) { hit(1900);  }
unsafe extern "C" fn s1901(_a0: usize) -> f32 { hit(1901); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1902(_a0: usize, _a1: f32) { hit(1902);  }
unsafe extern "C" fn s1903(_a0: usize) -> f32 { hit(1903); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1904(_a0: usize, _a1: i32) { hit(1904);  }
unsafe extern "C" fn s1905(_a0: usize) -> i32 { hit(1905); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1906(_a0: usize) -> f32 { hit(1906); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1907(_a0: usize) -> f32 { hit(1907); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1908(_a0: usize, _a1: f32, _a2: f32) { hit(1908);  }
unsafe extern "C" fn s1909(_a0: usize, _a1: i32) { hit(1909);  }
unsafe extern "C" fn s1910(_a0: usize) -> i32 { hit(1910); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1911(_a0: usize, _a1: f32) { hit(1911);  }
unsafe extern "C" fn s1912(_a0: usize) -> f32 { hit(1912); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1913(_a0: usize, _a1: f32) { hit(1913);  }
unsafe extern "C" fn s1914(_a0: usize) -> f32 { hit(1914); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1915(_a0: usize, _a1: i32) { hit(1915);  }
unsafe extern "C" fn s1916(_a0: usize) -> i32 { hit(1916); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1917(_a0: usize, _a1: f32) { hit(1917);  }
unsafe extern "C" fn s1918(_a0: usize) -> f32 { hit(1918); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1919(_a0: usize, _a1: f32) { hit(1919);  }
unsafe extern "C" fn s1920(_a0: usize) -> f32 { hit(1920); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1921(_a0: usize, _a1: f32) { hit(1921);  }
unsafe extern "C" fn s1922(_a0: usize) -> f32 { hit(1922); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1923(_a0: usize, _a1: i32) { hit(1923);  }
unsafe extern "C" fn s1924(_a0: usize) -> i32 { hit(1924); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1925(_a0: usize) -> f32 { hit(1925); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1926(_a0: usize) -> f32 { hit(1926); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1927(_a0: usize, _a1: f32, _a2: f32) { hit(1927);  }
unsafe extern "C" fn s1928(_a0: usize, _a1: f32) { hit(1928);  }
unsafe extern "C" fn s1929(_a0: usize) -> f32 { hit(1929); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1930(_a0: usize) -> f32 { hit(1930); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1931(_a0: usize) -> f32 { hit(1931); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1932(_a0: usize) -> f32 { hit(1932); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1933(_a0: usize) -> f32 { hit(1933); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1934(_a0: usize, _a1: f32) { hit(1934);  }
unsafe extern "C" fn s1935(_a0: usize) -> f32 { hit(1935); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1936(_a0: usize, _a1: f32) { hit(1936);  }
unsafe extern "C" fn s1937(_a0: usize) -> f32 { hit(1937); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1938(_a0: usize, _a1: f32) { hit(1938);  }
unsafe extern "C" fn s1939(_a0: usize) -> f32 { hit(1939); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1940(_a0: usize, _a1: u8) { hit(1940);  }
unsafe extern "C" fn s1941(_a0: usize, _a1: u8) { hit(1941);  }
unsafe extern "C" fn s1942(_a0: usize) -> u8 { hit(1942); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1943(_a0: usize, _a1: u32) -> i32 { hit(1943); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1944(_a0: usize, _a1: u32) -> i32 { hit(1944); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1945(_a0: usize, _a1: u32) -> i32 { hit(1945); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1946(_a0: usize) -> i32 { hit(1946); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1947(_a0: usize) -> i32 { hit(1947); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1948(_a0: usize, _a1: S_Sandbox_StringToken) { hit(1948);  }
unsafe extern "C" fn s1949(_a0: usize, _a1: i32, _a2: i32) { hit(1949);  }
unsafe extern "C" fn s1950(_a0: usize) -> usize { hit(1950); crate::trace::object(1950) }
unsafe extern "C" fn s1951(_a0: usize) -> i64 { hit(1951); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1952(_a0: usize, _a1: i32, _a2: usize, _a3: i32, _a4: usize) { hit(1952);  }
unsafe extern "C" fn s1953(_a0: usize, _a1: usize, _a2: usize, _a3: i32, _a4: i32, _a5: i32, _a6: i32, _a7: f32, _a8: f32) { hit(1953);  }
unsafe extern "C" fn s1954(_a0: usize, _a1: usize, _a2: f32) { hit(1954);  }
unsafe extern "C" fn s1955(_a0: usize, _a1: usize, _a2: usize, _a3: f32) { hit(1955);  }
unsafe extern "C" fn s1956(_a0: usize, _a1: usize, _a2: usize, _a3: i32, _a4: usize) { hit(1956);  }
unsafe extern "C" fn s1957(_a0: usize) -> i32 { hit(1957); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1958(_a0: usize, _a1: i32) { hit(1958);  }
unsafe extern "C" fn s1959(_a0: usize) -> i32 { hit(1959); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1960(_a0: usize, _a1: usize, _a2: usize) { hit(1960);  }
unsafe extern "C" fn s1961(_a0: usize, _a1: usize, _a2: usize, _a3: usize) { hit(1961);  }
unsafe extern "C" fn s1962(_a0: usize, _a1: usize) { hit(1962);  }
unsafe extern "C" fn s1963(_a0: usize) -> S_Sandbox_Sphere { hit(1963); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1964(_a0: usize) -> S_Capsule { hit(1964); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1965(_a0: usize, _a1: usize, _a2: usize, _a3: usize) { hit(1965);  }
unsafe extern "C" fn s1966(_a0: usize, _a1: f32) { hit(1966);  }
unsafe extern "C" fn s1967(_a0: usize) -> f32 { hit(1967); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1968(_a0: usize, _a1: usize) { hit(1968);  }
unsafe extern "C" fn s1969(_a0: usize) -> S_Vector3 { hit(1969); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1970(_a0: usize, _a1: f32) { hit(1970);  }
unsafe extern "C" fn s1971(_a0: usize, _a1: f32) { hit(1971);  }
unsafe extern "C" fn s1972(_a0: usize, _a1: i32) { hit(1972);  }
unsafe extern "C" fn s1973(_a0: usize, _a1: i32) { hit(1973);  }
unsafe extern "C" fn s1974(_a0: usize) -> S_BBox { hit(1974); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1975(_a0: usize) -> S_BBox { hit(1975); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1976(_a0: usize, _a1: usize, _a2: i32) -> i32 { hit(1976); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1977(_a0: usize) { hit(1977);  }
unsafe extern "C" fn s1978(_a0: usize) -> i32 { hit(1978); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1979(_a0: usize, _a1: usize) -> i32 { hit(1979); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1980(_a0: usize, _a1: usize) { hit(1980);  }
unsafe extern "C" fn s1981(_a0: usize) -> i32 { hit(1981); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1982(_a0: usize, _a1: usize) { hit(1982);  }
unsafe extern "C" fn s1983(_a0: usize, _a1: usize) { hit(1983);  }
unsafe extern "C" fn s1984(_a0: usize, _a1: usize) { hit(1984);  }
unsafe extern "C" fn s1985(_a0: usize) -> S_Vector3 { hit(1985); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1986(_a0: usize, _a1: i64) { hit(1986);  }
unsafe extern "C" fn s1987(_a0: usize) -> i64 { hit(1987); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1988(_a0: usize) { hit(1988);  }
unsafe extern "C" fn s1989(_a0: usize) { hit(1989);  }
unsafe extern "C" fn s1990(_a0: usize) -> i32 { hit(1990); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1991(_a0: usize, _a1: f32) { hit(1991);  }
unsafe extern "C" fn s1992(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize) -> i32 { hit(1992); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1993(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize) -> i32 { hit(1993); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1994(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize) -> i32 { hit(1994); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1995(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize) -> i32 { hit(1995); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1996(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize) -> i32 { hit(1996); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1997(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize) -> i32 { hit(1997); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1998(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize) -> i32 { hit(1998); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s1999(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize) -> i32 { hit(1999); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2000(_a0: usize, _a1: usize, _a2: usize) -> i32 { hit(2000); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2001(_a0: usize, _a1: usize) { hit(2001);  }
unsafe extern "C" fn s2002(_a0: usize, _a1: f32, _a2: i32) { hit(2002);  }
unsafe extern "C" fn s2003(_a0: usize, _a1: usize) { hit(2003);  }
unsafe extern "C" fn s2004(_a0: usize, _a1: usize) { hit(2004);  }
unsafe extern "C" fn s2005(_a0: usize, _a1: usize, _a2: usize, _a3: u64, _a4: i64) -> i32 { hit(2005); crate::handles::create(2005, "PhysAggregate") }
unsafe extern "C" fn s2006(_a0: usize, _a1: usize, _a2: usize, _a3: u64, _a4: i64) -> i32 { hit(2006); crate::handles::create(2006, "PhysAggregate") }
unsafe extern "C" fn s2007(_a0: usize, _a1: usize) { hit(2007);  }
unsafe extern "C" fn s2008(_a0: usize) -> i32 { hit(2008); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2009(_a0: usize, _a1: usize) { hit(2009);  }
unsafe extern "C" fn s2010(_a0: usize) -> i32 { hit(2010); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2011(_a0: usize, _a1: usize, _a2: usize, _a3: f32, _a4: u16) { hit(2011);  }
unsafe extern "C" fn s2012(_a0: usize, _a1: usize, _a2: usize, _a3: u16) { hit(2012);  }
unsafe extern "C" fn s2013(_a0: usize, _a1: usize, _a2: usize, _a3: i32, _a4: u16) { hit(2013);  }
unsafe extern "C" fn s2014(_a0: usize) { hit(2014);  }
unsafe extern "C" fn s2015(_a0: usize) -> i32 { hit(2015); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2016(_a0: usize, _a1: usize) { hit(2016);  }
unsafe extern "C" fn s2017(_a0: usize) -> i32 { hit(2017); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2018(_a0: usize) -> usize { hit(2018); crate::trace::object(2018) }
unsafe extern "C" fn s2019(_a0: usize) -> i64 { hit(2019); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2020(_a0: usize) { hit(2020);  }
unsafe extern "C" fn s2021(_a0: usize, _a1: usize, _a2: usize) { hit(2021);  }
unsafe extern "C" fn s2022(_a0: usize, _a1: usize, _a2: usize) -> i32 { hit(2022); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2023(_a0: usize, _a1: usize, _a2: usize) -> i32 { hit(2023); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2024(_a0: usize, _a1: i64, _a2: i32, _a3: i32) { hit(2024);  }
unsafe extern "C" fn s2025(_a0: usize, _a1: i64, _a2: i32, _a3: i32, _a4: i32) { hit(2025);  }
unsafe extern "C" fn s2026(_a0: usize, _a1: i64, _a2: i32, _a3: i32, _a4: i32, _a5: i32) { hit(2026);  }
unsafe extern "C" fn s2027(_a0: usize, _a1: i64, _a2: i32, _a3: i32, _a4: i32, _a5: i32, _a6: i32) { hit(2027);  }
unsafe extern "C" fn s2028(_a0: usize, _a1: i64, _a2: usize, _a3: u32, _a4: u32, _a5: u32) { hit(2028);  }
unsafe extern "C" fn s2029(_a0: usize, _a1: i64, _a2: usize, _a3: u32, _a4: u32, _a5: u32) { hit(2029);  }
unsafe extern "C" fn s2030(_a0: usize) -> usize { hit(2030); crate::trace::object(2030) }
unsafe extern "C" fn s2031(_a0: usize, _a1: usize) { hit(2031);  }
unsafe extern "C" fn s2032(_a0: usize, _a1: usize, _a2: i32, _a3: i64, _a4: i64, _a5: i64, _a6: i64, _a7: i64) { hit(2032);  }
unsafe extern "C" fn s2033(_a0: usize, _a1: usize, _a2: i64, _a3: i64, _a4: i64, _a5: i64) { hit(2033);  }
unsafe extern "C" fn s2034(_a0: usize, _a1: usize) { hit(2034);  }
unsafe extern "C" fn s2035(_a0: usize) -> usize { hit(2035); crate::trace::object(2035) }
unsafe extern "C" fn s2036(_a0: usize, _a1: usize) { hit(2036);  }
unsafe extern "C" fn s2037(_a0: usize, _a1: usize, _a2: i32, _a3: i32, _a4: i32) { hit(2037);  }
unsafe extern "C" fn s2038(_a0: usize, _a1: usize, _a2: S_Color32) { hit(2038);  }
unsafe extern "C" fn s2039(_a0: usize, _a1: usize, _a2: u32) { hit(2039);  }
unsafe extern "C" fn s2040(_a0: usize, _a1: usize, _a2: usize, _a3: usize) { hit(2040);  }
unsafe extern "C" fn s2041(_a0: usize, _a1: usize) { hit(2041);  }
unsafe extern "C" fn s2042(_a0: usize, _a1: usize) { hit(2042);  }
unsafe extern "C" fn s2043(_a0: usize, _a1: usize) { hit(2043);  }
unsafe extern "C" fn s2044(_a0: usize, _a1: usize) { hit(2044);  }
unsafe extern "C" fn s2045(_a0: usize, _a1: i32, _a2: i32, _a3: i32, _a4: i32) { hit(2045);  }
unsafe extern "C" fn s2046(_a0: usize) -> S_NativeEngine_RenderViewport { hit(2046); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2047(_a0: usize) { hit(2047);  }
unsafe extern "C" fn s2048(_a0: usize) { hit(2048);  }
unsafe extern "C" fn s2049(_a0: usize, _a1: usize, _a2: i32, _a3: i32) { hit(2049);  }
unsafe extern "C" fn s2050(_a0: usize, _a1: usize, _a2: i32) -> i32 { hit(2050); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2051(_a0: usize, _a1: usize, _a2: i32, _a3: i32) -> i32 { hit(2051); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2052(_a0: usize, _a1: i32, _a2: usize, _a3: i32) -> i32 { hit(2052); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2053(_a0: usize, _a1: i32, _a2: usize, _a3: i32, _a4: i32) -> i32 { hit(2053); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2054(_a0: usize, _a1: i32, _a2: usize, _a3: i32) -> i32 { hit(2054); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2055(_a0: usize, _a1: i32, _a2: usize, _a3: i32, _a4: i32) -> i32 { hit(2055); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2056(_a0: usize, _a1: usize, _a2: usize) { hit(2056);  }
unsafe extern "C" fn s2057(_a0: usize, _a1: usize) { hit(2057);  }
unsafe extern "C" fn s2058(_a0: usize, _a1: i64, _a2: usize, _a3: i32, _a4: i32) { hit(2058);  }
unsafe extern "C" fn s2059(_a0: usize, _a1: i32, _a2: usize) { hit(2059);  }
unsafe extern "C" fn s2060(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: i32, _a5: i32, _a6: i32) { hit(2060);  }
unsafe extern "C" fn s2061(_a0: usize, _a1: usize, _a2: usize, _a3: i32, _a4: i32, _a5: i32) { hit(2061);  }
unsafe extern "C" fn s2062(_a0: usize, _a1: usize) { hit(2062);  }
unsafe extern "C" fn s2063(_a0: usize) { hit(2063);  }
unsafe extern "C" fn s2064(_a0: usize, _a1: usize) { hit(2064);  }
unsafe extern "C" fn s2065(_a0: usize, _a1: S_Sandbox_StringToken) { hit(2065);  }
unsafe extern "C" fn s2066(_a0: usize, _a1: i64) { hit(2066);  }
unsafe extern "C" fn s2067(_a0: usize, _a1: i64) { hit(2067);  }
unsafe extern "C" fn s2068(_a0: usize, _a1: i64) { hit(2068);  }
unsafe extern "C" fn s2069(_a0: usize, _a1: i64) { hit(2069);  }
unsafe extern "C" fn s2070(_a0: usize) -> i64 { hit(2070); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2071(_a0: usize) -> i64 { hit(2071); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2072(_a0: usize) -> usize { hit(2072); crate::trace::object(2072) }
unsafe extern "C" fn s2073(_a0: usize, _a1: usize) { hit(2073);  }
unsafe extern "C" fn s2074(_a0: usize, _a1: usize) { hit(2074);  }
unsafe extern "C" fn s2075(_a0: usize) -> usize { hit(2075); crate::trace::object(2075) }
unsafe extern "C" fn s2076(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize, _a3: i64, _a4: u32) { hit(2076);  }
unsafe extern "C" fn s2077(_a0: usize, _a1: i32) { hit(2077);  }
unsafe extern "C" fn s2078(_a0: usize, _a1: f32) { hit(2078);  }
unsafe extern "C" fn s2079(_a0: usize, _a1: i32) { hit(2079);  }
unsafe extern "C" fn s2080(_a0: usize, _a1: usize) { hit(2080);  }
unsafe extern "C" fn s2081(_a0: usize, _a1: f32) { hit(2081);  }
unsafe extern "C" fn s2082(_a0: usize, _a1: S_Sandbox_StringToken) { hit(2082);  }
unsafe extern "C" fn s2083(_a0: usize) { hit(2083);  }
unsafe extern "C" fn s2084(_a0: usize, _a1: usize, _a2: i32) { hit(2084);  }
unsafe extern "C" fn s2085(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize) -> usize { hit(2085); crate::trace::object(2085) }
unsafe extern "C" fn s2086(_a0: usize, _a1: S_Sandbox_StringToken) -> usize { hit(2086); crate::trace::object(2086) }
unsafe extern "C" fn s2087(_a0: usize) -> usize { hit(2087); crate::trace::object(2087) }
unsafe extern "C" fn s2088(_a0: usize) -> usize { hit(2088); crate::trace::object(2088) }
unsafe extern "C" fn s2089(_a0: usize, _a1: usize, _a2: usize) { hit(2089);  }
unsafe extern "C" fn s2090(_a0: usize, _a1: usize, _a2: usize, _a3: i64) { hit(2090);  }
unsafe extern "C" fn s2091(_a0: usize, _a1: i32) { hit(2091);  }
unsafe extern "C" fn s2092(_a0: usize) -> i64 { hit(2092); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2093(_a0: usize, _a1: i64) { hit(2093);  }
unsafe extern "C" fn s2094(_a0: usize) -> i64 { hit(2094); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2095(_a0: usize, _a1: i64) { hit(2095);  }
unsafe extern "C" fn s2096(_a0: usize) -> S_NativeEngine_RenderViewport { hit(2096); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2097(_a0: usize, _a1: usize) { hit(2097);  }
unsafe extern "C" fn s2098(_a0: usize) -> i32 { hit(2098); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2099(_a0: usize, _a1: i32) { hit(2099);  }
unsafe extern "C" fn s2100(_a0: usize) -> S_NativeEngine_RenderViewport { hit(2100); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2101(_a0: usize) -> usize { hit(2101); crate::trace::object(2101) }
unsafe extern "C" fn s2102(_a0: usize) -> i32 { hit(2102); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2103(_a0: usize, _a1: usize) { hit(2103);  }
unsafe extern "C" fn s2104(_a0: usize) -> usize { hit(2104); crate::trace::object(2104) }
unsafe extern "C" fn s2105(_a0: usize, _a1: usize, _a2: usize, _a3: S_Sandbox_StringToken, _a4: usize) -> usize { hit(2105); crate::trace::object(2105) }
unsafe extern "C" fn s2106(_a0: usize, _a1: usize) -> usize { hit(2106); crate::trace::object(2106) }
unsafe extern "C" fn s2107(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: i32) -> usize { hit(2107); crate::trace::object(2107) }
unsafe extern "C" fn s2108(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize, _a6: usize, _a7: i32, _a8: i32) { hit(2108);  }
unsafe extern "C" fn s2109(_a0: usize) -> i32 { hit(2109); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2110(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize, _a6: i64) -> usize { hit(2110); crate::trace::object(2110) }
unsafe extern "C" fn s2111(_a0: usize, _a1: i64) { hit(2111);  }
unsafe extern "C" fn s2112(_a0: usize, _a1: i64) { hit(2112);  }
unsafe extern "C" fn s2113(_a0: usize) -> i64 { hit(2113); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2114(_a0: usize) -> i64 { hit(2114); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2115(_a0: usize, _a1: usize) { hit(2115);  }
unsafe extern "C" fn s2116(_a0: usize) -> S_Vector3 { hit(2116); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2117(_a0: usize, _a1: usize, _a2: usize, _a3: i32) -> usize { hit(2117); crate::trace::object(2117) }
unsafe extern "C" fn s2118(_a0: usize, _a1: usize) { hit(2118);  }
unsafe extern "C" fn s2119(_a0: usize) -> usize { hit(2119); crate::trace::object(2119) }
unsafe extern "C" fn s2120(_a0: usize) -> i32 { hit(2120); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2121(_a0: usize, _a1: i32) { hit(2121);  }
unsafe extern "C" fn s2122(_a0: usize) -> usize { hit(2122); crate::trace::object(2122) }
unsafe extern "C" fn s2123(_a0: usize) -> i32 { hit(2123); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2124(_a0: usize) -> i32 { hit(2124); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2125(_a0: usize) -> i32 { hit(2125); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2126(_a0: usize, _a1: i32) { hit(2126);  }
unsafe extern "C" fn s2127(_a0: usize) -> i32 { hit(2127); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2128(_a0: usize, _a1: i32) { hit(2128);  }
unsafe extern "C" fn s2129(_a0: usize) { hit(2129);  }
unsafe extern "C" fn s2130(_a0: usize) { hit(2130);  }
unsafe extern "C" fn s2131(_a0: usize) -> i32 { hit(2131); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2132(_a0: usize) -> i32 { hit(2132); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2133(_a0: usize) -> usize { hit(2133); crate::trace::object(2133) }
unsafe extern "C" fn s2134(_a0: usize, _a1: i32) { hit(2134);  }
unsafe extern "C" fn s2135(_a0: usize) -> i32 { hit(2135); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2136(_a0: usize) { hit(2136);  }
unsafe extern "C" fn s2137(_a0: usize, _a1: usize, _a2: usize) -> i32 { hit(2137); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2138(_a0: usize, _a1: usize, _a2: usize) -> i32 { hit(2138); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2139(_a0: usize) -> usize { hit(2139); crate::trace::object(2139) }
unsafe extern "C" fn s2140(_a0: usize, _a1: usize) { hit(2140);  }
unsafe extern "C" fn s2141(_a0: usize, _a1: usize) { hit(2141);  }
unsafe extern "C" fn s2142(_a0: usize, _a1: usize) { hit(2142);  }
unsafe extern "C" fn s2143(_a0: usize, _a1: usize, _a2: usize, _a3: f32) { hit(2143);  }
unsafe extern "C" fn s2144(_a0: usize) -> S_Vector4 { hit(2144); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2145(_a0: usize) { hit(2145);  }
unsafe extern "C" fn s2146(_a0: usize, _a1: usize) { hit(2146);  }
unsafe extern "C" fn s2147(_a0: usize, _a1: i32) -> i32 { hit(2147); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2148(_a0: usize) -> i32 { hit(2148); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2149(_a0: usize, _a1: i32) -> i32 { hit(2149); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2150(_a0: usize) -> i32 { hit(2150); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2151(_a0: usize) -> i32 { hit(2151); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2152(_a0: usize, _a1: i32) -> i32 { hit(2152); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2153(_a0: usize) -> usize { hit(2153); crate::trace::object(2153) }
unsafe extern "C" fn s2154(_a0: usize) -> usize { hit(2154); crate::trace::object(2154) }
unsafe extern "C" fn s2155(_a0: usize) -> i32 { hit(2155); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2156(_a0: usize) -> i32 { hit(2156); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2157(_a0: usize) -> usize { hit(2157); crate::trace::object(2157) }
unsafe extern "C" fn s2158(_a0: usize, _a1: i32) -> usize { hit(2158); crate::trace::object(2158) }
unsafe extern "C" fn s2159(_a0: usize, _a1: u64, _a2: i32, _a3: i32) -> usize { hit(2159); crate::trace::object(2159) }
unsafe extern "C" fn s2160(_a0: usize, _a1: u64) -> u64 { hit(2160); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2161(_a0: usize) -> usize { hit(2161); crate::trace::object(2161) }
unsafe extern "C" fn s2162(_a0: usize, _a1: usize, _a2: usize) -> i32 { hit(2162); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2163(_a0: usize) { hit(2163);  }
unsafe extern "C" fn s2164(_a0: usize, _a1: usize) { hit(2164); unsafe { crate::impls::ISteamGameServer_SetServerName(_a0, _a1) } }
unsafe extern "C" fn s2165(_a0: usize, _a1: usize) { hit(2165); unsafe { crate::impls::ISteamGameServer_SetMapName(_a0, _a1) } }
unsafe extern "C" fn s2166(_a0: usize, _a1: usize) { hit(2166);  }
unsafe extern "C" fn s2167(_a0: usize, _a1: i32) { hit(2167);  }
unsafe extern "C" fn s2168(_a0: usize, _a1: i32) { hit(2168);  }
unsafe extern "C" fn s2169(_a0: usize, _a1: i32) { hit(2169); unsafe { crate::impls::ISteamGameServer_SetMaxPlayerCount(_a0, _a1) } }
unsafe extern "C" fn s2170(_a0: usize, _a1: i32) { hit(2170);  }
unsafe extern "C" fn s2171(_a0: usize) { hit(2171); unsafe { crate::impls::ISteamGameServer_LogOnAnonymous(_a0) } }
unsafe extern "C" fn s2172(_a0: usize, _a1: usize) { hit(2172); unsafe { crate::impls::ISteamGameServer_LogOn(_a0, _a1) } }
unsafe extern "C" fn s2173(_a0: usize) { hit(2173); unsafe { crate::impls::ISteamGameServer_LogOff(_a0) } }
unsafe extern "C" fn s2174(_a0: usize, _a1: usize) { hit(2174);  }
unsafe extern "C" fn s2175(_a0: usize, _a1: usize) { hit(2175);  }
unsafe extern "C" fn s2176(_a0: usize, _a1: usize) { hit(2176);  }
unsafe extern "C" fn s2177(_a0: usize) -> i32 { hit(2177); unsafe { crate::impls::ISteamGameServer_BLoggedOn(_a0) } }
unsafe extern "C" fn s2178(_a0: usize) -> usize { hit(2178); crate::trace::object(2178) }
unsafe extern "C" fn s2179(_a0: usize, _a1: u64, _a2: usize, _a3: usize) -> S_Steamworks_Data_HAuthTicket { hit(2179); unsafe { crate::impls::ISteamGameServer_GetAuthSessionTicket(_a0, _a1, _a2, _a3) } }
unsafe extern "C" fn s2180(_a0: usize, _a1: u64, _a2: usize, _a3: i32) -> i64 { hit(2180); unsafe { crate::impls::ISteamGameServer_BeginAuthSession(_a0, _a1, _a2, _a3) } }
unsafe extern "C" fn s2181(_a0: usize, _a1: S_Steamworks_Data_HAuthTicket) { hit(2181);  }
unsafe extern "C" fn s2182(_a0: usize, _a1: u64) { hit(2182); unsafe { crate::impls::ISteamGameServer_EndAuthSession(_a0, _a1) } }
unsafe extern "C" fn s2183(_a0: usize) -> i32 { hit(2183); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2184(_a0: usize) -> i32 { hit(2184); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2185(_a0: usize, _a1: usize, _a2: usize) -> u64 { hit(2185); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2186(_a0: usize, _a1: u32) { hit(2186);  }
unsafe extern "C" fn s2187(_a0: usize, _a1: u32, _a2: usize, _a3: usize) { hit(2187);  }
unsafe extern "C" fn s2188(_a0: usize, _a1: u32, _a2: usize, _a3: usize) { hit(2188);  }
unsafe extern "C" fn s2189(_a0: usize, _a1: u32, _a2: usize) { hit(2189);  }
unsafe extern "C" fn s2190(_a0: usize, _a1: u32, _a2: u32, _a3: u32) { hit(2190);  }
unsafe extern "C" fn s2191(_a0: usize, _a1: u32, _a2: u32, _a3: u32) { hit(2191);  }
unsafe extern "C" fn s2192(_a0: usize, _a1: u32, _a2: u32) { hit(2192);  }
unsafe extern "C" fn s2193(_a0: usize, _a1: u32, _a2: u32) { hit(2193);  }
unsafe extern "C" fn s2194(_a0: usize, _a1: u32, _a2: i32) { hit(2194);  }
unsafe extern "C" fn s2195(_a0: usize, _a1: u32, _a2: i32) { hit(2195);  }
unsafe extern "C" fn s2196(_a0: usize, _a1: u32, _a2: i32) { hit(2196);  }
unsafe extern "C" fn s2197(_a0: usize, _a1: u32, _a2: i32) { hit(2197);  }
unsafe extern "C" fn s2198(_a0: usize, _a1: u32, _a2: f32) { hit(2198);  }
unsafe extern "C" fn s2199(_a0: usize, _a1: u32, _a2: u32, _a3: i32, _a4: i32) { hit(2199);  }
unsafe extern "C" fn s2200(_a0: usize, _a1: u32, _a2: u32, _a3: i32) { hit(2200);  }
unsafe extern "C" fn s2201(_a0: usize, _a1: u32, _a2: u32, _a3: i32) { hit(2201);  }
unsafe extern "C" fn s2202(_a0: usize, _a1: u32, _a2: i32) { hit(2202);  }
unsafe extern "C" fn s2203(_a0: usize, _a1: u32, _a2: i32) { hit(2203);  }
unsafe extern "C" fn s2204(_a0: usize, _a1: u32, _a2: i32) { hit(2204);  }
unsafe extern "C" fn s2205(_a0: usize, _a1: u32, _a2: i32, _a3: i32) { hit(2205);  }
unsafe extern "C" fn s2206(_a0: usize, _a1: u32, _a2: i32) { hit(2206);  }
unsafe extern "C" fn s2207(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: u32, _a6: i32, _a7: i32) { hit(2207);  }
unsafe extern "C" fn s2208(_a0: usize, _a1: u64) { hit(2208);  }
unsafe extern "C" fn s2209(_a0: usize, _a1: u64) -> i32 { hit(2209); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2210(_a0: usize, _a1: u64, _a2: i32) -> u64 { hit(2210); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2211(_a0: usize, _a1: u64, _a2: usize) -> usize { hit(2211); crate::trace::object(2211) }
unsafe extern "C" fn s2212(_a0: usize, _a1: u64, _a2: usize, _a3: usize) -> i32 { hit(2212); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2213(_a0: usize, _a1: u64) -> i32 { hit(2213); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2214(_a0: usize, _a1: u64, _a2: usize) { hit(2214);  }
unsafe extern "C" fn s2215(_a0: usize, _a1: u64, _a2: i32, _a3: usize, _a4: i32, _a5: usize, _a6: i32) -> i32 { hit(2215); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2216(_a0: usize, _a1: u64, _a2: usize, _a3: usize) -> i32 { hit(2216); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2217(_a0: usize, _a1: u64, _a2: usize, _a3: i32, _a4: i32, _a5: i32) -> i32 { hit(2217); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2218(_a0: usize, _a1: i32, _a2: usize, _a3: i32) -> i32 { hit(2218); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2219(_a0: usize, _a1: u64) -> i32 { hit(2219); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2220(_a0: usize, _a1: u64, _a2: i32) -> i32 { hit(2220); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2221(_a0: usize, _a1: usize) { hit(2221);  }
unsafe extern "C" fn s2222(_a0: usize, _a1: u64) -> S_Steamworks_Data_SteamNetConnectionRealTimeStatus_t { hit(2222); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2223(_a0: usize, _a1: S_Sandbox_Network_HSteamNetConnection) -> S_Steamworks_Data_SteamNetConnectionRealTimeStatus_t { hit(2223); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2224(_a0: usize) { hit(2224);  }
unsafe extern "C" fn s2225(_a0: usize) { hit(2225);  }
unsafe extern "C" fn s2226(_a0: usize, _a1: usize) { hit(2226); unsafe { crate::impls::StmNtwrkngSckts_ReleaseMessage(_a0, _a1) } }
unsafe extern "C" fn s2227(_a0: usize, _a1: i64, _a2: i32) { hit(2227);  }
unsafe extern "C" fn s2228(_a0: usize, _a1: i64, _a2: usize) { hit(2228);  }
unsafe extern "C" fn s2229(_a0: usize) { hit(2229);  }
unsafe extern "C" fn s2230(_a0: usize) -> usize { hit(2230); crate::trace::object(2230) }
unsafe extern "C" fn s2231(_a0: usize, _a1: usize) -> i32 { hit(2231); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2232(_a0: usize, _a1: usize) -> i32 { hit(2232); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2233(_a0: usize, _a1: usize) -> i32 { hit(2233); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2234(_a0: usize, _a1: usize, _a2: usize, _a3: i32) -> i32 { hit(2234); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2235(_a0: usize, _a1: usize, _a2: usize, _a3: i32) -> i32 { hit(2235); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2236(_a0: usize, _a1: usize, _a2: usize, _a3: u32) -> u64 { hit(2236); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2237(_a0: usize, _a1: usize) -> i32 { hit(2237); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2238(_a0: usize) -> i32 { hit(2238); unsafe { crate::impls::ISteamUser_BLoggedOn(_a0) } }
unsafe extern "C" fn s2239(_a0: usize) -> u64 { hit(2239); unsafe { crate::impls::ISteamUser_GetSteamID(_a0) } }
unsafe extern "C" fn s2240(_a0: usize, _a1: u64, _a2: usize, _a3: usize) -> S_Steamworks_Data_HAuthTicket { hit(2240); unsafe { crate::impls::ISteamUser_GetAuthSessionTicket(_a0, _a1, _a2, _a3) } }
unsafe extern "C" fn s2241(_a0: usize, _a1: u64, _a2: usize, _a3: i32) -> i64 { hit(2241); unsafe { crate::impls::ISteamUser_BeginAuthSession(_a0, _a1, _a2, _a3) } }
unsafe extern "C" fn s2242(_a0: usize, _a1: S_Steamworks_Data_HAuthTicket) { hit(2242);  }
unsafe extern "C" fn s2243(_a0: usize, _a1: u64) { hit(2243); unsafe { crate::impls::ISteamUser_EndAuthSession(_a0, _a1) } }
unsafe extern "C" fn s2244(_a0: usize, _a1: u32) -> i32 { hit(2244); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2245(_a0: usize, _a1: i64, _a2: u64, _a3: usize, _a4: usize, _a5: u32) -> i32 { hit(2245); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2246(_a0: usize, _a1: usize) { hit(2246);  }
unsafe extern "C" fn s2247(_a0: usize) { hit(2247);  }
unsafe extern "C" fn s2248(_a0: usize, _a1: usize) { hit(2248);  }
unsafe extern "C" fn s2249(_a0: usize, _a1: usize, _a2: u64, _a3: u64, _a4: usize, _a5: i64, _a6: i64, _a7: i32, _a8: u32) -> usize { hit(2249); crate::trace::object(2249) }
unsafe extern "C" fn s2250(_a0: usize) { hit(2250);  }
unsafe extern "C" fn s2251(_a0: usize) -> usize { hit(2251); crate::trace::object(2251) }
unsafe extern "C" fn s2252(_a0: usize) { hit(2252);  }
unsafe extern "C" fn s2253(_a0: usize, _a1: usize, _a2: usize) { hit(2253);  }
unsafe extern "C" fn s2254(_a0: usize) { hit(2254);  }
unsafe extern "C" fn s2255(_a0: usize) -> i32 { hit(2255); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2256(_a0: usize) -> i32 { hit(2256); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2257(_a0: usize) -> i32 { hit(2257); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2258(_a0: usize, _a1: usize, _a2: usize) -> i32 { hit(2258); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2259(_a0: usize) -> i32 { hit(2259); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2260(_a0: usize, _a1: u32) { hit(2260);  }
unsafe extern "C" fn s2261(_a0: usize) -> usize { hit(2261); crate::trace::object(2261) }
unsafe extern "C" fn s2262(_a0: usize, _a1: usize) -> i32 { hit(2262); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2263(_a0: usize, _a1: usize, _a2: i32) -> usize { hit(2263); crate::trace::object(2263) }
unsafe extern "C" fn s2264(_a0: usize, _a1: usize) { hit(2264);  }
unsafe extern "C" fn s2265(_a0: i32, _a1: i32, _a2: i32, _a3: i64, _a4: i32) -> i32 { hit(2265); unsafe { crate::impls::ImageLoader_GetMemRequired(_a0, _a1, _a2, _a3, _a4) } }
unsafe extern "C" fn s2266(_a0: i32, _a1: i32, _a2: i32, _a3: i32, _a4: i64) -> i32 { hit(2266); unsafe { crate::impls::ImageLoader_GetMemRequired_1(_a0, _a1, _a2, _a3, _a4) } }
unsafe extern "C" fn s2267(_a0: usize, _a1: i64, _a2: usize, _a3: i64, _a4: i32, _a5: i32, _a6: i32, _a7: i32) -> i32 { hit(2267); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2268(_a0: usize) { hit(2268);  }
unsafe extern "C" fn s2269() -> usize { hit(2269); crate::trace::object(2269) }
unsafe extern "C" fn s2270(_a0: usize) -> i32 { hit(2270); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2271(_a0: usize) -> i32 { hit(2271); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2272(_a0: usize) -> i64 { hit(2272); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2273(_a0: usize) -> i32 { hit(2273); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2274(_a0: usize) -> i32 { hit(2274); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2275(_a0: usize) -> i64 { hit(2275); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2276(_a0: usize) -> u64 { hit(2276); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2277(_a0: usize) -> f32 { hit(2277); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2278(_a0: usize) -> f64 { hit(2278); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2279(_a0: usize) -> usize { hit(2279); crate::trace::object(2279) }
unsafe extern "C" fn s2280(_a0: usize) -> S_Vector3 { hit(2280); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2281(_a0: usize) -> S_Color32 { hit(2281); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2282(_a0: usize, _a1: i32) { hit(2282);  }
unsafe extern "C" fn s2283(_a0: usize, _a1: usize) { hit(2283);  }
unsafe extern "C" fn s2284(_a0: usize, _a1: usize) { hit(2284);  }
unsafe extern "C" fn s2285(_a0: usize, _a1: i32) { hit(2285);  }
unsafe extern "C" fn s2286(_a0: usize, _a1: f32) { hit(2286);  }
unsafe extern "C" fn s2287(_a0: usize, _a1: usize, _a2: usize) { hit(2287);  }
unsafe extern "C" fn s2288(_a0: usize, _a1: usize, _a2: i32) { hit(2288);  }
unsafe extern "C" fn s2289(_a0: usize, _a1: usize, _a2: f32) { hit(2289);  }
unsafe extern "C" fn s2290(_a0: usize, _a1: usize) -> usize { hit(2290); crate::trace::object(2290) }
unsafe extern "C" fn s2291(_a0: usize, _a1: usize, _a2: i32) -> i32 { hit(2291); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2292(_a0: usize, _a1: usize, _a2: f32) -> f32 { hit(2292); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2293(_a0: usize, _a1: usize, _a2: usize) -> S_Vector3 { hit(2293); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2294(_a0: usize) { hit(2294);  }
unsafe extern "C" fn s2295(_a0: usize) -> i32 { hit(2295); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2296(_a0: usize) -> usize { hit(2296); crate::trace::object(2296) }
unsafe extern "C" fn s2297(_a0: usize, _a1: i32) -> usize { hit(2297); crate::trace::object(2297) }
unsafe extern "C" fn s2298(_a0: usize, _a1: usize) -> usize { hit(2298); crate::trace::object(2298) }
unsafe extern "C" fn s2299(_a0: usize) { hit(2299);  }
unsafe extern "C" fn s2300(_a0: usize) -> i32 { hit(2300); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2301(_a0: usize, _a1: i32) -> usize { hit(2301); crate::trace::object(2301) }
unsafe extern "C" fn s2302(_a0: usize, _a1: i32) -> usize { hit(2302); crate::trace::object(2302) }
unsafe extern "C" fn s2303(_a0: usize) -> usize { hit(2303); crate::trace::object(2303) }
unsafe extern "C" fn s2304(_a0: i32) -> i32 { hit(2304); unsafe { crate::impls::LZ4Glue_CompressBound(_a0) } }
unsafe extern "C" fn s2305(_a0: usize, _a1: usize, _a2: i32, _a3: i32) -> i32 { hit(2305); unsafe { crate::impls::LZ4Glue_Compress(_a0, _a1, _a2, _a3) } }
unsafe extern "C" fn s2306(_a0: usize, _a1: usize, _a2: i32, _a3: i32, _a4: i32) -> i32 { hit(2306); unsafe { crate::impls::LZ4Glue_CompressHC(_a0, _a1, _a2, _a3, _a4) } }
unsafe extern "C" fn s2307(_a0: usize, _a1: usize, _a2: i32, _a3: i32) -> i32 { hit(2307); unsafe { crate::impls::LZ4Glue_Decompress(_a0, _a1, _a2, _a3) } }
unsafe extern "C" fn s2308(_a0: i32) -> i32 { hit(2308); unsafe { crate::impls::LZ4Glue_CompressFrameBound(_a0) } }
unsafe extern "C" fn s2309(_a0: usize, _a1: usize, _a2: i32, _a3: i32, _a4: i32) -> i32 { hit(2309); unsafe { crate::impls::LZ4Glue_CompressFrame(_a0, _a1, _a2, _a3, _a4) } }
unsafe extern "C" fn s2310(_a0: usize, _a1: i32) -> i64 { hit(2310); unsafe { crate::impls::LZ4Glue_GetFrameContentSize(_a0, _a1) } }
unsafe extern "C" fn s2311(_a0: usize, _a1: usize, _a2: i32, _a3: i32) -> i32 { hit(2311); unsafe { crate::impls::LZ4Glue_DecompressFrame(_a0, _a1, _a2, _a3) } }
unsafe extern "C" fn s2312(_a0: usize, _a1: i32, _a2: usize) -> usize { hit(2312); crate::trace::object(2312) }
unsafe extern "C" fn s2313(_a0: usize) -> i32 { hit(2313); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2314(_a0: usize, _a1: usize, _a2: u32) { hit(2314);  }
unsafe extern "C" fn s2315(_a0: usize) -> i32 { hit(2315); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2316(_a0: usize, _a1: usize, _a2: u32) { hit(2316);  }
unsafe extern "C" fn s2317(_a0: usize, _a1: i32, _a2: i32) -> i32 { hit(2317); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2318(_a0: usize, _a1: i32, _a2: i32) -> i32 { hit(2318); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2319(_a0: usize, _a1: i32, _a2: i32) -> i32 { hit(2319); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2320(_a0: usize, _a1: i32, _a2: i32, _a3: usize, _a4: u32, _a5: usize, _a6: u32, _a7: usize) -> i32 { hit(2320); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2321(_a0: usize, _a1: i32) -> i32 { hit(2321); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2322(_a0: usize, _a1: i32) -> i32 { hit(2322); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2323(_a0: usize, _a1: i32) -> i32 { hit(2323); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2324(_a0: usize, _a1: i32) -> i32 { hit(2324); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2325(_a0: usize, _a1: usize) { hit(2325);  }
unsafe extern "C" fn s2326(_a0: usize, _a1: i32) { hit(2326);  }
unsafe extern "C" fn s2327(_a0: usize, _a1: usize, _a2: usize) { hit(2327);  }
unsafe extern "C" fn s2328(_a0: usize, _a1: f32) { hit(2328);  }
unsafe extern "C" fn s2329(_a0: usize, _a1: i32, _a2: i32) { hit(2329);  }
unsafe extern "C" fn s2330(_a0: usize, _a1: i32, _a2: i32) { hit(2330);  }
unsafe extern "C" fn s2331(_a0: usize, _a1: usize, _a2: i32, _a3: i32, _a4: i32, _a5: i32, _a6: i32) -> i32 { hit(2331); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2332(_a0: usize, _a1: usize, _a2: usize, _a3: i32) { hit(2332);  }
unsafe extern "C" fn s2333(_a0: usize, _a1: usize, _a2: usize, _a3: i32) { hit(2333);  }
unsafe extern "C" fn s2334(_a0: i32, _a1: i32, _a2: usize, _a3: usize, _a4: i32, _a5: usize, _a6: i32) -> usize { hit(2334); crate::trace::object(2334) }
unsafe extern "C" fn s2335(_a0: i32, _a1: usize, _a2: usize) -> usize { hit(2335); crate::trace::object(2335) }
unsafe extern "C" fn s2336(_a0: i32, _a1: i32, _a2: usize, _a3: i32) -> usize { hit(2336); crate::trace::object(2336) }
unsafe extern "C" fn s2337(_a0: usize, _a1: i32, _a2: i32) -> usize { hit(2337); crate::trace::object(2337) }
unsafe extern "C" fn s2338(_a0: usize, _a1: usize, _a2: i32, _a3: i32) { hit(2338);  }
unsafe extern "C" fn s2339(_a0: usize, _a1: i32, _a2: i32) -> usize { hit(2339); crate::trace::object(2339) }
unsafe extern "C" fn s2340(_a0: usize, _a1: usize, _a2: i32, _a3: i32) { hit(2340);  }
unsafe extern "C" fn s2341(_a0: usize, _a1: usize, _a2: i32, _a3: i32) { hit(2341);  }
unsafe extern "C" fn s2342(_a0: usize, _a1: usize, _a2: i32, _a3: i32) { hit(2342);  }
unsafe extern "C" fn s2343(_a0: usize, _a1: i32) -> usize { hit(2343); crate::trace::object(2343) }
unsafe extern "C" fn s2344(_a0: usize, _a1: i32) -> usize { hit(2344); crate::trace::object(2344) }
unsafe extern "C" fn s2345(_a0: usize, _a1: i32, _a2: usize, _a3: i32) -> i32 { hit(2345); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2346(_a0: usize, _a1: i32, _a2: usize, _a3: usize, _a4: usize) { hit(2346);  }
unsafe extern "C" fn s2347(_a0: i32, _a1: usize, _a2: i32, _a3: usize, _a4: usize, _a5: i64, _a6: i64, _a7: i32, _a8: i32, _a9: usize) { hit(2347);  }
unsafe extern "C" fn s2348(_a0: usize, _a1: i32) -> usize { hit(2348); crate::trace::object(2348) }
unsafe extern "C" fn s2349(_a0: usize, _a1: i32) -> usize { hit(2349); crate::trace::object(2349) }
unsafe extern "C" fn s2350(_a0: usize) -> usize { hit(2350); crate::trace::object(2350) }
unsafe extern "C" fn s2351(_a0: usize) -> usize { hit(2351); crate::trace::object(2351) }
unsafe extern "C" fn s2352(_a0: usize) -> usize { hit(2352); crate::trace::object(2352) }
unsafe extern "C" fn s2353(_a0: usize) -> usize { hit(2353); crate::trace::object(2353) }
unsafe extern "C" fn s2354(_a0: usize, _a1: usize, _a2: i32, _a3: usize, _a4: i32, _a5: usize) { hit(2354);  }
unsafe extern "C" fn s2355(_a0: usize, _a1: usize, _a2: i64) { hit(2355);  }
unsafe extern "C" fn s2356(_a0: usize, _a1: i32, _a2: u32) -> u64 { hit(2356); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2357(_a0: usize, _a1: usize, _a2: u32, _a3: usize, _a4: i32, _a5: usize) { hit(2357);  }
unsafe extern "C" fn s2358() { hit(2358);  }
unsafe extern "C" fn s2359(_a0: usize) -> S_PhysicsTrace_Result { hit(2359); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2360(_a0: usize, _a1: usize) { hit(2360);  }
unsafe extern "C" fn s2361(_a0: usize, _a1: usize, _a2: usize) -> S_PhysicsTrace_Result { hit(2361); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2362(_a0: usize, _a1: usize, _a2: usize) -> S_PhysicsTrace_Result { hit(2362); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2363(_a0: usize, _a1: usize, _a2: usize) -> S_PhysicsTrace_Result { hit(2363); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2364(_a0: usize, _a1: usize) -> i64 { hit(2364); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2365(_a0: usize, _a1: usize) -> i32 { hit(2365); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2366(_a0: usize, _a1: usize) -> i32 { hit(2366); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2367(_a0: usize, _a1: usize) -> i32 { hit(2367); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2368(_a0: usize, _a1: f32, _a2: f32, _a3: usize, _a4: i32, _a5: i32, _a6: i32, _a7: i32) { hit(2368);  }
unsafe extern "C" fn s2369() -> i32 { hit(2369); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2370(_a0: i32) -> usize { hit(2370); crate::trace::object(2370) }
unsafe extern "C" fn s2371(_a0: usize) -> S_Vector4 { hit(2371); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2372(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize) { hit(2372);  }
unsafe extern "C" fn s2373(_a0: usize, _a1: usize, _a2: i32, _a3: i32, _a4: i64, _a5: i64, _a6: i32) -> usize { hit(2373); crate::trace::object(2373) }
unsafe extern "C" fn s2374(_a0: i64) -> i32 { hit(2374); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2375() -> i64 { hit(2375); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2376() -> i64 { hit(2376); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2377(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize) -> i32 { hit(2377); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2378(_a0: usize, _a1: i64, _a2: usize, _a3: usize, _a4: i32, _a5: usize, _a6: i32, _a7: usize) { hit(2378);  }
unsafe extern "C" fn s2379(_a0: usize, _a1: usize, _a2: usize) { hit(2379);  }
unsafe extern "C" fn s2380(_a0: usize, _a1: usize, _a2: usize) { hit(2380);  }
unsafe extern "C" fn s2381(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize, _a6: usize) { hit(2381);  }
unsafe extern "C" fn s2382(_a0: usize, _a1: usize, _a2: S_Sandbox_StringToken, _a3: usize, _a4: usize, _a5: i32, _a6: usize) { hit(2382);  }
unsafe extern "C" fn s2383(_a0: usize, _a1: usize, _a2: usize, _a3: i32, _a4: i32) -> i32 { hit(2383); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2384(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: i32, _a5: i32, _a6: i32, _a7: f32, _a8: i32, _a9: i32, _a10: i32) -> i32 { hit(2384); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2385(_a0: usize, _a1: i32, _a2: usize, _a3: usize) -> i32 { hit(2385); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2386(_a0: usize, _a1: i32, _a2: usize, _a3: usize, _a4: i32) -> i32 { hit(2386); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2387(_a0: i32) -> i32 { hit(2387); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2388() -> usize { hit(2388); crate::trace::object(2388) }
unsafe extern "C" fn s2389(_a0: usize, _a1: usize, _a2: usize, _a3: i32, _a4: usize, _a5: usize, _a6: usize, _a7: i32, _a8: i32, _a9: usize, _a10: usize, _a11: usize, _a12: usize, _a13: usize, _a14: i32) -> i32 { hit(2389); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2390(_a0: usize, _a1: usize, _a2: i32, _a3: i32, _a4: i32) -> i32 { hit(2390); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2391(_a0: usize, _a1: usize, _a2: i32) { hit(2391);  }
unsafe extern "C" fn s2392(_a0: usize, _a1: usize, _a2: usize, _a3: i32) { hit(2392);  }
unsafe extern "C" fn s2393(_a0: usize, _a1: usize, _a2: i32, _a3: i32, _a4: i32, _a5: i32, _a6: i32) -> i32 { hit(2393); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2394(_a0: usize, _a1: usize, _a2: i32, _a3: i32, _a4: i32, _a5: i32) { hit(2394);  }
unsafe extern "C" fn s2395(_a0: usize, _a1: usize) -> i32 { hit(2395); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2396(_a0: usize, _a1: i32, _a2: i32, _a3: usize) -> i32 { hit(2396); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2397(_a0: usize) -> u64 { hit(2397); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2398(_a0: usize, _a1: usize, _a2: usize) { hit(2398);  }
unsafe extern "C" fn s2399(_a0: usize, _a1: usize, _a2: S_Sandbox_StringToken) -> i32 { hit(2399); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2400(_a0: usize, _a1: i32) -> i32 { hit(2400); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2401(_a0: usize, _a1: usize, _a2: i32) -> i32 { hit(2401); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2402(_a0: usize) { hit(2402);  }
unsafe extern "C" fn s2403(_a0: i32, _a1: usize) { hit(2403);  }
unsafe extern "C" fn s2404(_a0: i32) -> usize { hit(2404); crate::trace::object(2404) }
unsafe extern "C" fn s2405(_a0: usize, _a1: usize, _a2: S_Sandbox_StringToken, _a3: usize, _a4: usize, _a5: i32) { hit(2405);  }
unsafe extern "C" fn s2406(_a0: usize) { hit(2406);  }
unsafe extern "C" fn s2407(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: i32) -> i32 { hit(2407); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2408(_a0: usize, _a1: usize) -> i32 { hit(2408); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2409(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: i32, _a6: f32, _a7: f32, _a8: f32, _a9: f32) -> i32 { hit(2409); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2410(_a0: usize, _a1: i32, _a2: usize, _a3: usize, _a4: usize) -> usize { hit(2410); crate::trace::object(2410) }
unsafe extern "C" fn s2411(_a0: usize, _a1: i32) -> f32 { hit(2411); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2412(_a0: usize, _a1: usize, _a2: usize, _a3: i32, _a4: i32, _a5: i32, _a6: i32) { hit(2412);  }
unsafe extern "C" fn s2413(_a0: usize, _a1: usize, _a2: usize, _a3: S_Sandbox_StringToken, _a4: usize, _a5: i32, _a6: i32, _a7: i32, _a8: f32, _a9: i32, _a10: i32, _a11: usize, _a12: i32, _a13: i32, _a14: usize) -> i32 { hit(2413); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2414(_a0: usize, _a1: usize, _a2: usize, _a3: S_Sandbox_StringToken, _a4: usize, _a5: i32, _a6: i32, _a7: i32, _a8: f32, _a9: i32, _a10: i32, _a11: usize, _a12: i32, _a13: usize, _a14: i32, _a15: usize) -> i32 { hit(2414); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2415() -> i32 { hit(2415); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2416(_a0: usize) { hit(2416);  }
unsafe extern "C" fn s2417(_a0: usize) -> u64 { hit(2417); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2418(_a0: usize, _a1: usize, _a2: usize) -> i32 { hit(2418); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2419(_a0: usize, _a1: usize) -> i32 { hit(2419); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2420(_a0: usize, _a1: usize, _a2: S_Sandbox_StringToken, _a3: usize, _a4: usize, _a5: i32, _a6: usize) { hit(2420);  }
unsafe extern "C" fn s2421(_a0: usize, _a1: usize, _a2: S_Sandbox_StringToken, _a3: usize, _a4: usize, _a5: i32, _a6: usize, _a7: i32) { hit(2421);  }
unsafe extern "C" fn s2422(_a0: usize, _a1: usize, _a2: S_Sandbox_StringToken, _a3: usize, _a4: usize, _a5: i32, _a6: usize, _a7: i32) { hit(2422);  }
unsafe extern "C" fn s2423(_a0: usize, _a1: usize, _a2: S_Sandbox_StringToken, _a3: usize, _a4: usize, _a5: usize, _a6: i32, _a7: usize, _a8: i32) { hit(2423);  }
unsafe extern "C" fn s2424(_a0: usize, _a1: usize, _a2: usize, _a3: i32, _a4: i32, _a5: i32) { hit(2424);  }
unsafe extern "C" fn s2425(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: u32) { hit(2425);  }
unsafe extern "C" fn s2426(_a0: usize, _a1: usize, _a2: usize, _a3: u32, _a4: u32, _a5: u32) { hit(2426);  }
unsafe extern "C" fn s2427(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: u32) { hit(2427);  }
unsafe extern "C" fn s2428(_a0: usize, _a1: S_Sandbox_StringToken, _a2: usize, _a3: usize, _a4: i32) { hit(2428);  }
unsafe extern "C" fn s2429(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: i32, _a5: i32, _a6: u32, _a7: u32, _a8: u32, _a9: u32) { hit(2429);  }
unsafe extern "C" fn s2430(_a0: usize, _a1: usize, _a2: usize, _a3: u32, _a4: u32) { hit(2430);  }
unsafe extern "C" fn s2431(_a0: usize, _a1: usize, _a2: u32) { hit(2431);  }
unsafe extern "C" fn s2432(_a0: usize, _a1: usize, _a2: usize, _a3: u32, _a4: u32, _a5: u32) { hit(2432);  }
unsafe extern "C" fn s2433(_a0: usize, _a1: usize, _a2: usize, _a3: u32) { hit(2433);  }
unsafe extern "C" fn s2434(_a0: usize, _a1: usize, _a2: u32) { hit(2434);  }
unsafe extern "C" fn s2435(_a0: usize, _a1: usize, _a2: usize) { hit(2435);  }
unsafe extern "C" fn s2436(_a0: usize) -> S_Capsule { hit(2436); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2437(_a0: usize, _a1: usize) { hit(2437);  }
unsafe extern "C" fn s2438(_a0: usize) -> u32 { hit(2438); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2439(_a0: usize, _a1: u32) { hit(2439);  }
unsafe extern "C" fn s2440(_a0: usize) -> u32 { hit(2440); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2441(_a0: usize, _a1: u32) { hit(2441);  }
unsafe extern "C" fn s2442(_a0: usize) -> usize { hit(2442); crate::trace::object(2442) }
unsafe extern "C" fn s2443(_a0: usize) -> u32 { hit(2443); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2444(_a0: usize, _a1: u32) { hit(2444);  }
unsafe extern "C" fn s2445(_a0: usize) -> u32 { hit(2445); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2446(_a0: usize, _a1: u32) { hit(2446);  }
unsafe extern "C" fn s2447(_a0: usize) -> i32 { hit(2447); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2448(_a0: usize) -> i32 { hit(2448); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2449(_a0: usize) -> f32 { hit(2449); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2450(_a0: usize) -> S_Vector3 { hit(2450); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2451(_a0: usize) -> S_Vector3 { hit(2451); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2452(_a0: usize) -> f32 { hit(2452); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2453(_a0: usize) -> i32 { hit(2453); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2454(_a0: usize) -> S_BBox { hit(2454); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2455(_a0: usize, _a1: i32) -> S_Vector3 { hit(2455); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2456(_a0: usize, _a1: i32, _a2: usize, _a3: usize) { hit(2456);  }
unsafe extern "C" fn s2457(_a0: usize) -> usize { hit(2457); crate::trace::object(2457) }
unsafe extern "C" fn s2458(_a0: usize) -> u32 { hit(2458); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2459(_a0: usize, _a1: u32) { hit(2459);  }
unsafe extern "C" fn s2460(_a0: usize) -> u32 { hit(2460); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2461(_a0: usize, _a1: u32) { hit(2461);  }
unsafe extern "C" fn s2462(_a0: usize) -> i32 { hit(2462); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2463(_a0: usize) -> i32 { hit(2463); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2464(_a0: usize, _a1: i32) -> S_Vector3 { hit(2464); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2465(_a0: usize) -> i32 { hit(2465); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2466(_a0: usize) -> i32 { hit(2466); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2467(_a0: usize) -> S_BBox { hit(2467); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2468(_a0: usize) -> i32 { hit(2468); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2469(_a0: usize, _a1: i32) -> i32 { hit(2469); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2470(_a0: usize, _a1: i32, _a2: usize, _a3: usize, _a4: usize) { hit(2470);  }
unsafe extern "C" fn s2471(_a0: usize, _a1: i32, _a2: usize, _a3: usize, _a4: usize) { hit(2471);  }
unsafe extern "C" fn s2472(_a0: usize, _a1: usize, _a2: usize) { hit(2472);  }
unsafe extern "C" fn s2473(_a0: usize) -> S_Sandbox_Sphere { hit(2473); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2474(_a0: usize, _a1: usize) { hit(2474);  }
unsafe extern "C" fn s2475(_a0: usize) -> u32 { hit(2475); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2476(_a0: usize, _a1: u32) { hit(2476);  }
unsafe extern "C" fn s2477(_a0: usize) -> u32 { hit(2477); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2478(_a0: usize, _a1: u32) { hit(2478);  }
unsafe extern "C" fn s2479(_a0: usize) -> u32 { hit(2479); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2480(_a0: usize, _a1: u32) { hit(2480);  }
unsafe extern "C" fn s2481(_a0: usize) -> u32 { hit(2481); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2482(_a0: usize, _a1: u32) { hit(2482);  }
unsafe extern "C" fn s2483(_a0: usize) -> u32 { hit(2483); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2484(_a0: usize, _a1: u32) { hit(2484);  }
unsafe extern "C" fn s2485(_a0: usize) -> u32 { hit(2485); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2486(_a0: usize, _a1: u32) { hit(2486);  }
unsafe extern "C" fn s2487(_a0: usize) -> u32 { hit(2487); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2488(_a0: usize, _a1: u32) { hit(2488);  }
unsafe extern "C" fn s2489(_a0: usize) -> u32 { hit(2489); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2490(_a0: usize, _a1: u32) { hit(2490);  }
unsafe extern "C" fn s2491(_a0: usize) -> u32 { hit(2491); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2492(_a0: usize, _a1: u32) { hit(2492);  }
unsafe extern "C" fn s2493(_a0: usize) -> u32 { hit(2493); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2494(_a0: usize, _a1: u32) { hit(2494);  }
unsafe extern "C" fn s2495(_a0: usize) -> u32 { hit(2495); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2496(_a0: usize, _a1: u32) { hit(2496);  }
unsafe extern "C" fn s2497(_a0: usize) -> u32 { hit(2497); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2498(_a0: usize, _a1: u32) { hit(2498);  }
unsafe extern "C" fn s2499(_a0: usize) -> u32 { hit(2499); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2500(_a0: usize, _a1: u32) { hit(2500);  }
unsafe extern "C" fn s2501(_a0: usize) -> u32 { hit(2501); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2502(_a0: usize, _a1: u32) { hit(2502);  }
unsafe extern "C" fn s2503(_a0: usize) -> u32 { hit(2503); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2504(_a0: usize, _a1: u32) { hit(2504);  }
unsafe extern "C" fn s2505(_a0: usize) -> u32 { hit(2505); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2506(_a0: usize, _a1: u32) { hit(2506);  }
unsafe extern "C" fn s2507(_a0: usize) -> u32 { hit(2507); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2508(_a0: usize, _a1: u32) { hit(2508);  }
unsafe extern "C" fn s2509(_a0: usize) -> u32 { hit(2509); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2510(_a0: usize, _a1: u32) { hit(2510);  }
unsafe extern "C" fn s2511(_a0: usize) -> u32 { hit(2511); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2512(_a0: usize, _a1: u32) { hit(2512);  }
unsafe extern "C" fn s2513(_a0: usize) -> u32 { hit(2513); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2514(_a0: usize, _a1: u32) { hit(2514);  }
unsafe extern "C" fn s2515(_a0: usize) -> u32 { hit(2515); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2516(_a0: usize, _a1: u32) { hit(2516);  }
unsafe extern "C" fn s2517(_a0: usize) -> u32 { hit(2517); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2518(_a0: usize, _a1: u32) { hit(2518);  }
unsafe extern "C" fn s2519(_a0: usize) -> u32 { hit(2519); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2520(_a0: usize, _a1: u32) { hit(2520);  }
unsafe extern "C" fn s2521(_a0: usize) -> u32 { hit(2521); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2522(_a0: usize, _a1: u32) { hit(2522);  }
unsafe extern "C" fn s2523(_a0: usize) -> u32 { hit(2523); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2524(_a0: usize, _a1: u32) { hit(2524);  }
unsafe extern "C" fn s2525(_a0: usize) -> u32 { hit(2525); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2526(_a0: usize, _a1: u32) { hit(2526);  }
unsafe extern "C" fn s2527(_a0: usize) -> u32 { hit(2527); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2528(_a0: usize, _a1: u32) { hit(2528);  }
unsafe extern "C" fn s2529(_a0: usize) -> u32 { hit(2529); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2530(_a0: usize, _a1: u32) { hit(2530);  }
unsafe extern "C" fn s2531(_a0: usize) -> u32 { hit(2531); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2532(_a0: usize, _a1: u32) { hit(2532);  }
unsafe extern "C" fn s2533(_a0: usize) -> u32 { hit(2533); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2534(_a0: usize, _a1: u32) { hit(2534);  }
unsafe extern "C" fn s2535(_a0: usize) -> u32 { hit(2535); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2536(_a0: usize, _a1: u32) { hit(2536);  }
unsafe extern "C" fn s2537(_a0: usize) -> u32 { hit(2537); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2538(_a0: usize, _a1: u32) { hit(2538);  }
unsafe extern "C" fn s2539(_a0: usize) -> u32 { hit(2539); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2540(_a0: usize, _a1: u32) { hit(2540);  }
unsafe extern "C" fn s2541(_a0: usize) -> u32 { hit(2541); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2542(_a0: usize, _a1: u32) { hit(2542);  }
unsafe extern "C" fn s2543(_a0: usize) -> u32 { hit(2543); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2544(_a0: usize, _a1: u32) { hit(2544);  }
unsafe extern "C" fn s2545(_a0: usize) -> u32 { hit(2545); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2546(_a0: usize, _a1: u32) { hit(2546);  }
unsafe extern "C" fn s2547(_a0: usize) -> u32 { hit(2547); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2548(_a0: usize, _a1: u32) { hit(2548);  }
unsafe extern "C" fn s2549(_a0: usize) -> i32 { hit(2549); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2550(_a0: usize, _a1: i32) { hit(2550);  }
unsafe extern "C" fn s2551(_a0: usize) -> u32 { hit(2551); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2552(_a0: usize, _a1: u32) { hit(2552);  }
unsafe extern "C" fn s2553(_a0: usize) -> u32 { hit(2553); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2554(_a0: usize, _a1: u32) { hit(2554);  }
unsafe extern "C" fn s2555(_a0: usize) -> u64 { hit(2555); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2556(_a0: usize, _a1: u64) { hit(2556);  }
unsafe extern "C" fn s2557(_a0: usize) -> u32 { hit(2557); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2558(_a0: usize, _a1: u32) { hit(2558);  }
unsafe extern "C" fn s2559(_a0: usize) -> u32 { hit(2559); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2560(_a0: usize, _a1: u32) { hit(2560);  }
unsafe extern "C" fn s2561(_a0: usize) -> u32 { hit(2561); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2562(_a0: usize, _a1: u32) { hit(2562);  }
unsafe extern "C" fn s2563(_a0: usize) -> u32 { hit(2563); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2564(_a0: usize, _a1: u32) { hit(2564);  }
unsafe extern "C" fn s2565(_a0: usize) -> u32 { hit(2565); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2566(_a0: usize, _a1: u32) { hit(2566);  }
unsafe extern "C" fn s2567(_a0: usize) -> u32 { hit(2567); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2568(_a0: usize, _a1: u32) { hit(2568);  }
unsafe extern "C" fn s2569(_a0: usize) -> u32 { hit(2569); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2570(_a0: usize, _a1: u32) { hit(2570);  }
unsafe extern "C" fn s2571(_a0: usize) -> u32 { hit(2571); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2572(_a0: usize, _a1: u32) { hit(2572);  }
unsafe extern "C" fn s2573(_a0: usize) -> u32 { hit(2573); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2574(_a0: usize, _a1: u32) { hit(2574);  }
unsafe extern "C" fn s2575(_a0: usize) -> u32 { hit(2575); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2576(_a0: usize, _a1: u32) { hit(2576);  }
unsafe extern "C" fn s2577(_a0: usize) -> u32 { hit(2577); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2578(_a0: usize, _a1: u32) { hit(2578);  }
unsafe extern "C" fn s2579(_a0: usize) -> u32 { hit(2579); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2580(_a0: usize, _a1: u32) { hit(2580);  }
unsafe extern "C" fn s2581(_a0: usize) -> u32 { hit(2581); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2582(_a0: usize, _a1: u32) { hit(2582);  }
unsafe extern "C" fn s2583(_a0: usize) -> u32 { hit(2583); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2584(_a0: usize, _a1: u32) { hit(2584);  }
unsafe extern "C" fn s2585() -> i32 { hit(2585); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2586() -> i32 { hit(2586); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2587() -> i32 { hit(2587); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2588() -> i32 { hit(2588); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2589(_a0: usize) -> usize { hit(2589); crate::trace::object(2589) }
unsafe extern "C" fn s2590(_a0: usize, _a1: i64, _a2: i32) -> usize { hit(2590); crate::trace::object(2590) }
unsafe extern "C" fn s2591(_a0: usize) -> i32 { hit(2591); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2592(_a0: usize, _a1: i32) -> S_SheetFrameImage_t { hit(2592); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2593(_a0: usize) -> f32 { hit(2593); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2594(_a0: usize, _a1: f32) { hit(2594);  }
unsafe extern "C" fn s2595(_a0: usize) -> i32 { hit(2595); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2596(_a0: usize) -> u32 { hit(2596); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2597(_a0: usize, _a1: u32) { hit(2597);  }
unsafe extern "C" fn s2598(_a0: usize) -> i32 { hit(2598); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2599(_a0: usize, _a1: i32) { hit(2599);  }
unsafe extern "C" fn s2600(_a0: usize) -> i32 { hit(2600); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2601(_a0: usize, _a1: i32) { hit(2601);  }
unsafe extern "C" fn s2602(_a0: usize) -> i32 { hit(2602); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2603(_a0: usize, _a1: i32) { hit(2603);  }
unsafe extern "C" fn s2604(_a0: usize) -> i32 { hit(2604); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2605(_a0: usize, _a1: i32) { hit(2605);  }
unsafe extern "C" fn s2606(_a0: usize) -> f32 { hit(2606); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2607(_a0: usize, _a1: f32) { hit(2607);  }
unsafe extern "C" fn s2608() -> usize { hit(2608); crate::trace::object(2608) }
unsafe extern "C" fn s2609() -> i32 { hit(2609); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2610(_a0: i32) -> i32 { hit(2610); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2611(_a0: i32, _a1: usize) -> usize { hit(2611); crate::trace::object(2611) }
unsafe extern "C" fn s2612(_a0: i32, _a1: usize, _a2: usize) -> i32 { hit(2612); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2613() -> i32 { hit(2613); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2614() -> usize { hit(2614); crate::trace::object(2614) }
unsafe extern "C" fn s2615(_a0: usize, _a1: i32) { hit(2615);  }
unsafe extern "C" fn s2616() -> i32 { hit(2616); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2617() -> i32 { hit(2617); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2618(_a0: usize, _a1: u32) -> usize { hit(2618); crate::trace::object(2618) }
unsafe extern "C" fn s2619(_a0: usize, _a1: u32, _a2: i32, _a3: i32) -> i32 { hit(2619); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2620(_a0: usize, _a1: usize) -> usize { hit(2620); crate::trace::object(2620) }
unsafe extern "C" fn s2621(_a0: usize) { hit(2621);  }
unsafe extern "C" fn s2622(_a0: usize) -> usize { hit(2622); crate::trace::object(2622) }
unsafe extern "C" fn s2623(_a0: usize) -> i32 { hit(2623); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2624(_a0: usize, _a1: i32) { hit(2624);  }
unsafe extern "C" fn s2625(_a0: usize) -> i32 { hit(2625); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2626(_a0: usize, _a1: i32) { hit(2626);  }
unsafe extern "C" fn s2627(_a0: usize) -> i32 { hit(2627); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2628(_a0: usize, _a1: i32) { hit(2628);  }
unsafe extern "C" fn s2629() -> usize { hit(2629); crate::trace::object(2629) }
unsafe extern "C" fn s2630() -> usize { hit(2630); crate::trace::object(2630) }
unsafe extern "C" fn s2631(_a0: u64) -> i32 { hit(2631); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2632(_a0: u64) -> usize { hit(2632); crate::trace::object(2632) }
unsafe extern "C" fn s2633(_a0: u64, _a1: i32) -> i32 { hit(2633); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2634(_a0: u64) { hit(2634);  }
unsafe extern "C" fn s2635(_a0: u64) { hit(2635);  }
unsafe extern "C" fn s2636(_a0: u64) { hit(2636);  }
unsafe extern "C" fn s2637(_a0: u64) { hit(2637);  }
unsafe extern "C" fn s2638(_a0: usize) -> i32 { hit(2638); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2639() -> usize { hit(2639); crate::trace::object(2639) }
unsafe extern "C" fn s2640() -> usize { hit(2640); crate::trace::object(2640) }
unsafe extern "C" fn s2641(_a0: u64) -> usize { hit(2641); crate::trace::object(2641) }
unsafe extern "C" fn s2642(_a0: usize) { hit(2642);  }
unsafe extern "C" fn s2643(_a0: usize) -> u64 { hit(2643); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2644(_a0: usize, _a1: usize) { hit(2644);  }
unsafe extern "C" fn s2645(_a0: usize, _a1: usize) { hit(2645);  }
unsafe extern "C" fn s2646(_a0: usize, _a1: usize) { hit(2646);  }
unsafe extern "C" fn s2647(_a0: usize, _a1: usize) { hit(2647);  }
unsafe extern "C" fn s2648(_a0: usize, _a1: i32) { hit(2648);  }
unsafe extern "C" fn s2649(_a0: usize, _a1: usize) { hit(2649);  }
unsafe extern "C" fn s2650(_a0: usize, _a1: usize) { hit(2650);  }
unsafe extern "C" fn s2651(_a0: usize, _a1: usize) { hit(2651);  }
unsafe extern "C" fn s2652(_a0: usize, _a1: i32) { hit(2652);  }
unsafe extern "C" fn s2653(_a0: usize, _a1: usize, _a2: usize) { hit(2653);  }
unsafe extern "C" fn s2654(_a0: usize, _a1: usize) { hit(2654);  }
unsafe extern "C" fn s2655(_a0: usize) { hit(2655);  }
unsafe extern "C" fn s2656(_a0: usize, _a1: usize, _a2: i32) { hit(2656);  }
unsafe extern "C" fn s2657(_a0: usize, _a1: usize) { hit(2657);  }
unsafe extern "C" fn s2658(_a0: usize, _a1: u32, _a2: usize) { hit(2658);  }
unsafe extern "C" fn s2659(_a0: usize, _a1: u32, _a2: usize) { hit(2659);  }
unsafe extern "C" fn s2660(_a0: usize, _a1: u32) { hit(2660);  }
unsafe extern "C" fn s2661(_a0: usize, _a1: usize, _a2: usize) { hit(2661);  }
unsafe extern "C" fn s2662(_a0: usize, _a1: usize) -> i32 { hit(2662); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2663(_a0: usize) -> f32 { hit(2663); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2664(_a0: usize) -> u64 { hit(2664); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2665(_a0: usize) -> u64 { hit(2665); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2666(_a0: usize) -> i32 { hit(2666); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2667(_a0: usize) -> i32 { hit(2667); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2668(_a0: usize, _a1: i32) { hit(2668);  }
unsafe extern "C" fn s2669(_a0: usize) -> i32 { hit(2669); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2670(_a0: usize, _a1: i32) { hit(2670);  }
unsafe extern "C" fn s2671(_a0: usize) -> i32 { hit(2671); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2672(_a0: usize, _a1: i32) { hit(2672);  }
unsafe extern "C" fn s2673(_a0: usize) -> i32 { hit(2673); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2674(_a0: usize, _a1: i32) { hit(2674);  }
unsafe extern "C" fn s2675(_a0: usize) -> i32 { hit(2675); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2676(_a0: usize, _a1: i32) { hit(2676);  }
unsafe extern "C" fn s2677(_a0: usize) -> i32 { hit(2677); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2678(_a0: usize, _a1: i32) { hit(2678);  }
unsafe extern "C" fn s2679(_a0: usize) -> i32 { hit(2679); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2680(_a0: usize, _a1: i32) { hit(2680);  }
unsafe extern "C" fn s2681(_a0: u64) -> usize { hit(2681); crate::trace::object(2681) }
unsafe extern "C" fn s2682(_a0: u64, _a1: i32) -> usize { hit(2682); crate::trace::object(2682) }
unsafe extern "C" fn s2683(_a0: usize) { hit(2683);  }
unsafe extern "C" fn s2684(_a0: usize) -> i32 { hit(2684); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2685(_a0: usize, _a1: i32) { hit(2685);  }
unsafe extern "C" fn s2686(_a0: usize) -> i32 { hit(2686); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2687(_a0: usize, _a1: i32) { hit(2687);  }
unsafe extern "C" fn s2688(_a0: usize) -> i32 { hit(2688); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2689(_a0: usize, _a1: i32) { hit(2689);  }
unsafe extern "C" fn s2690(_a0: usize) -> i32 { hit(2690); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2691(_a0: usize, _a1: i32) { hit(2691);  }
unsafe extern "C" fn s2692(_a0: usize) -> i32 { hit(2692); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2693(_a0: usize, _a1: i32) { hit(2693);  }
unsafe extern "C" fn s2694(_a0: usize) -> i32 { hit(2694); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2695(_a0: usize, _a1: i32) -> usize { hit(2695); crate::trace::object(2695) }
unsafe extern "C" fn s2696(_a0: usize) -> i32 { hit(2696); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2697(_a0: usize, _a1: i32) -> usize { hit(2697); crate::trace::object(2697) }
unsafe extern "C" fn s2698(_a0: usize) -> i32 { hit(2698); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2699(_a0: usize, _a1: i32) -> usize { hit(2699); crate::trace::object(2699) }
unsafe extern "C" fn s2700(_a0: usize) -> i32 { hit(2700); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2701(_a0: usize, _a1: i32) -> usize { hit(2701); crate::trace::object(2701) }
unsafe extern "C" fn s2702(_a0: usize) -> i32 { hit(2702); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2703(_a0: usize, _a1: i32) -> u16 { hit(2703); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2704(_a0: usize) -> u32 { hit(2704); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2705(_a0: usize, _a1: u32) { hit(2705);  }
unsafe extern "C" fn s2706(_a0: usize) -> f32 { hit(2706); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2707(_a0: usize, _a1: f32) { hit(2707);  }
unsafe extern "C" fn s2708(_a0: usize) -> u16 { hit(2708); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2709(_a0: usize, _a1: u16) { hit(2709);  }
unsafe extern "C" fn s2710(_a0: usize) -> f32 { hit(2710); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2711(_a0: usize, _a1: f32) { hit(2711);  }
unsafe extern "C" fn s2712(_a0: usize) -> f32 { hit(2712); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2713(_a0: usize, _a1: f32) { hit(2713);  }
unsafe extern "C" fn s2714(_a0: usize) -> f32 { hit(2714); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2715(_a0: usize, _a1: f32) { hit(2715);  }
unsafe extern "C" fn s2716(_a0: usize) -> f32 { hit(2716); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2717(_a0: usize, _a1: f32) { hit(2717);  }
unsafe extern "C" fn s2718(_a0: usize) -> i32 { hit(2718); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2719(_a0: usize, _a1: i32) { hit(2719);  }
unsafe extern "C" fn s2720(_a0: usize) -> S_Vector3 { hit(2720); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2721(_a0: usize, _a1: usize) { hit(2721);  }
unsafe extern "C" fn s2722(_a0: usize) -> f32 { hit(2722); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2723(_a0: usize) -> f32 { hit(2723); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2724(_a0: usize) -> f32 { hit(2724); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2725(_a0: usize) -> f32 { hit(2725); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2726(_a0: usize) -> f32 { hit(2726); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2727(_a0: usize) -> f32 { hit(2727); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2728(_a0: usize, _a1: f32) { hit(2728);  }
unsafe extern "C" fn s2729(_a0: usize, _a1: f32) { hit(2729);  }
unsafe extern "C" fn s2730(_a0: usize, _a1: f32) { hit(2730);  }
unsafe extern "C" fn s2731(_a0: usize, _a1: f32) { hit(2731);  }
unsafe extern "C" fn s2732(_a0: usize, _a1: f32) { hit(2732);  }
unsafe extern "C" fn s2733(_a0: usize, _a1: f32) { hit(2733);  }
unsafe extern "C" fn s2734(_a0: usize) -> u16 { hit(2734); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2735(_a0: usize, _a1: u16) { hit(2735);  }
unsafe extern "C" fn s2736(_a0: usize) -> u16 { hit(2736); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2737(_a0: usize, _a1: u16) { hit(2737);  }
unsafe extern "C" fn s2738(_a0: usize) -> u16 { hit(2738); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2739(_a0: usize, _a1: u16) { hit(2739);  }
unsafe extern "C" fn s2740(_a0: usize) -> u16 { hit(2740); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2741(_a0: usize, _a1: u16) { hit(2741);  }
unsafe extern "C" fn s2742(_a0: usize) -> i32 { hit(2742); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2743(_a0: usize, _a1: i32) { hit(2743);  }
unsafe extern "C" fn s2744(_a0: usize) -> i32 { hit(2744); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2745(_a0: usize, _a1: i32) { hit(2745);  }
unsafe extern "C" fn s2746(_a0: usize) -> i32 { hit(2746); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2747(_a0: usize, _a1: i32) { hit(2747);  }
unsafe extern "C" fn s2748(_a0: usize) -> S_Vector3 { hit(2748); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2749(_a0: usize, _a1: usize) { hit(2749);  }
unsafe extern "C" fn s2750(_a0: usize) -> f32 { hit(2750); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2751(_a0: usize, _a1: f32) { hit(2751);  }
unsafe extern "C" fn s2752(_a0: usize) -> i32 { hit(2752); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2753(_a0: usize, _a1: i32) { hit(2753);  }
unsafe extern "C" fn s2754(_a0: usize) -> i32 { hit(2754); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2755(_a0: usize, _a1: i32) { hit(2755);  }
unsafe extern "C" fn s2756(_a0: usize) -> i32 { hit(2756); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2757(_a0: usize, _a1: i32) { hit(2757);  }
unsafe extern "C" fn s2758(_a0: usize) -> S_Vector3 { hit(2758); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2759(_a0: usize, _a1: usize) { hit(2759);  }
unsafe extern "C" fn s2760(_a0: usize) -> f32 { hit(2760); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2761(_a0: usize, _a1: f32) { hit(2761);  }
unsafe extern "C" fn s2762(_a0: usize) -> f32 { hit(2762); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2763(_a0: usize, _a1: f32) { hit(2763);  }
unsafe extern "C" fn s2764(_a0: usize) -> f32 { hit(2764); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2765(_a0: usize, _a1: f32) { hit(2765);  }
unsafe extern "C" fn s2766(_a0: usize) -> f32 { hit(2766); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2767(_a0: usize, _a1: f32) { hit(2767);  }
unsafe extern "C" fn s2768(_a0: usize) -> f32 { hit(2768); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2769(_a0: usize, _a1: f32) { hit(2769);  }
unsafe extern "C" fn s2770(_a0: usize) -> f32 { hit(2770); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2771(_a0: usize, _a1: f32) { hit(2771);  }
unsafe extern "C" fn s2772(_a0: usize) -> f32 { hit(2772); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2773(_a0: usize, _a1: f32) { hit(2773);  }
unsafe extern "C" fn s2774(_a0: usize) -> i32 { hit(2774); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2775(_a0: usize, _a1: i32) { hit(2775);  }
unsafe extern "C" fn s2776(_a0: usize) -> i32 { hit(2776); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2777(_a0: usize, _a1: i32) { hit(2777);  }
unsafe extern "C" fn s2778(_a0: usize) -> f32 { hit(2778); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2779(_a0: usize, _a1: f32) { hit(2779);  }
unsafe extern "C" fn s2780(_a0: usize) -> f32 { hit(2780); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2781(_a0: usize, _a1: f32) { hit(2781);  }
unsafe extern "C" fn s2782(_a0: usize) -> S_Rotation { hit(2782); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2783(_a0: usize, _a1: usize) { hit(2783);  }
unsafe extern "C" fn s2784(_a0: usize) -> i32 { hit(2784); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2785(_a0: usize, _a1: i32) { hit(2785);  }
unsafe extern "C" fn s2786(_a0: usize) -> f32 { hit(2786); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2787(_a0: usize, _a1: f32) { hit(2787);  }
unsafe extern "C" fn s2788(_a0: usize) -> S_Transform { hit(2788); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2789(_a0: usize, _a1: usize) { hit(2789);  }
unsafe extern "C" fn s2790(_a0: usize) -> S_Transform { hit(2790); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2791(_a0: usize, _a1: usize) { hit(2791);  }
unsafe extern "C" fn s2792(_a0: usize) { hit(2792);  }
unsafe extern "C" fn s2793(_a0: usize) -> i32 { hit(2793); 1 }
unsafe extern "C" fn s2794(_a0: usize) -> i32 { hit(2794); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2795(_a0: usize) -> i32 { hit(2795); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2796(_a0: usize) -> usize { hit(2796); crate::trace::object(2796) }
unsafe extern "C" fn s2797(_a0: usize) -> usize { hit(2797); crate::trace::object(2797) }
unsafe extern "C" fn s2798(_a0: usize) -> S_System_Guid { hit(2798); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2799(_a0: usize) -> i64 { hit(2799); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2800(_a0: usize) -> i32 { hit(2800); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2801(_a0: usize) -> i32 { hit(2801); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2802(_a0: usize) -> i32 { hit(2802); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2803(_a0: usize) -> i32 { hit(2803); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2804(_a0: usize) -> i32 { hit(2804); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2805(_a0: usize) -> f32 { hit(2805); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2806(_a0: usize, _a1: i32) -> usize { hit(2806); crate::trace::object(2806) }
unsafe extern "C" fn s2807(_a0: usize) { hit(2807);  }
unsafe extern "C" fn s2808(_a0: usize) { hit(2808);  }
unsafe extern "C" fn s2809(_a0: usize, _a1: usize, _a2: i32, _a3: u32, _a4: i32) { hit(2809);  }
unsafe extern "C" fn s2810(_a0: usize) { hit(2810);  }
unsafe extern "C" fn s2811(_a0: usize) -> i32 { hit(2811); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2812(_a0: usize, _a1: i32) { hit(2812);  }
unsafe extern "C" fn s2813(_a0: usize) { hit(2813);  }
unsafe extern "C" fn s2814(_a0: usize) -> usize { hit(2814); crate::trace::object(2814) }
unsafe extern "C" fn s2815(_a0: usize) -> usize { hit(2815); crate::trace::object(2815) }
unsafe extern "C" fn s2816(_a0: usize, _a1: usize) { hit(2816);  }
unsafe extern "C" fn s2817(_a0: usize) -> i32 { hit(2817); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2818(_a0: usize, _a1: i32) { hit(2818);  }
unsafe extern "C" fn s2819() -> i32 { hit(2819); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2820(_a0: i32) -> usize { hit(2820); crate::trace::object(2820) }
unsafe extern "C" fn s2821() -> usize { hit(2821); crate::trace::object(2821) }
unsafe extern "C" fn s2822(_a0: usize, _a1: i32) -> usize { hit(2822); crate::trace::object(2822) }
unsafe extern "C" fn s2823(_a0: usize) { hit(2823);  }
unsafe extern "C" fn s2824(_a0: usize, _a1: usize, _a2: i32) -> i32 { hit(2824); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2825(_a0: i32, _a1: i32) -> usize { hit(2825); crate::trace::object(2825) }
unsafe extern "C" fn s2826(_a0: usize) { hit(2826);  }
unsafe extern "C" fn s2827(_a0: usize, _a1: usize, _a2: i32, _a3: usize, _a4: i32) -> i32 { hit(2827); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2828(_a0: i32) -> usize { hit(2828); crate::trace::object(2828) }
unsafe extern "C" fn s2829(_a0: usize) { hit(2829);  }
unsafe extern "C" fn s2830(_a0: usize, _a1: usize, _a2: i32, _a3: usize, _a4: i32) -> i32 { hit(2830); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2831() -> usize { hit(2831); crate::trace::object(2831) }
unsafe extern "C" fn s2832(_a0: usize) { hit(2832);  }
unsafe extern "C" fn s2833(_a0: usize) { hit(2833);  }
unsafe extern "C" fn s2834(_a0: usize) -> usize { hit(2834); crate::trace::object(2834) }
unsafe extern "C" fn s2835(_a0: usize, _a1: usize) -> i64 { hit(2835); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2836(_a0: usize) -> i64 { hit(2836); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2837(_a0: usize) -> i64 { hit(2837); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2838(_a0: usize) -> u32 { hit(2838); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2839(_a0: usize) -> u32 { hit(2839); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2840(_a0: usize) -> u32 { hit(2840); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2841(_a0: usize) -> u32 { hit(2841); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2842(_a0: usize) -> f32 { hit(2842); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2843(_a0: usize, _a1: u32, _a2: usize) -> i64 { hit(2843); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2844(_a0: usize, _a1: u32, _a2: f32, _a3: f32, _a4: usize) -> i64 { hit(2844); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2845(_a0: usize) { hit(2845);  }
unsafe extern "C" fn s2846(_a0: usize, _a1: usize) -> i64 { hit(2846); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2847(_a0: usize) -> i64 { hit(2847); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2848(_a0: usize, _a1: usize, _a2: i64, _a3: usize) -> i64 { hit(2848); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2849(_a0: usize, _a1: usize, _a2: i64, _a3: usize) -> i64 { hit(2849); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2850(_a0: usize, _a1: usize, _a2: i64, _a3: usize) -> i64 { hit(2850); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2851(_a0: usize, _a1: usize, _a2: i64, _a3: usize) -> i64 { hit(2851); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2852(_a0: usize, _a1: f32, _a2: f32, _a3: f32, _a4: i64) -> i64 { hit(2852); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2853(_a0: usize, _a1: i64, _a2: i64, _a3: usize) -> i64 { hit(2853); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2854(_a0: usize) -> usize { hit(2854); crate::trace::object(2854) }
unsafe extern "C" fn s2855() -> i32 { hit(2855); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2856(_a0: usize, _a1: usize) -> usize { hit(2856); crate::trace::object(2856) }
unsafe extern "C" fn s2857(_a0: usize) -> usize { hit(2857); crate::trace::object(2857) }
unsafe extern "C" fn s2858(_a0: usize) -> usize { hit(2858); crate::trace::object(2858) }
unsafe extern "C" fn s2859(_a0: usize) -> usize { hit(2859); crate::trace::object(2859) }
unsafe extern "C" fn s2860(_a0: usize) -> S_Facepunch_XR_InstanceProperties { hit(2860); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2861() { hit(2861);  }
unsafe extern "C" fn s2862() -> usize { hit(2862); unsafe { crate::impls::g_pAudioDevice_Name() } }
unsafe extern "C" fn s2863() -> u32 { hit(2863); unsafe { crate::impls::g_pAudioDevice_ChannelCount() } }
unsafe extern "C" fn s2864() -> u32 { hit(2864); unsafe { crate::impls::g_pAudioDevice_MixChannelCount() } }
unsafe extern "C" fn s2865() -> u32 { hit(2865); unsafe { crate::impls::g_pAudioDevice_BitsPerSample() } }
unsafe extern "C" fn s2866() -> u32 { hit(2866); unsafe { crate::impls::g_pAudioDevice_BytesPerSample() } }
unsafe extern "C" fn s2867() -> u32 { hit(2867); unsafe { crate::impls::g_pAudioDevice_SampleRate() } }
unsafe extern "C" fn s2868() -> i32 { hit(2868); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2869() { hit(2869);  }
unsafe extern "C" fn s2870() { hit(2870);  }
unsafe extern "C" fn s2871(_a0: i32) { hit(2871);  }
unsafe extern "C" fn s2872() { hit(2872);  }
unsafe extern "C" fn s2873() { hit(2873);  }
unsafe extern "C" fn s2874() -> i32 { hit(2874); 1 }
unsafe extern "C" fn s2875(_a0: usize) { hit(2875);  }
unsafe extern "C" fn s2876(_a0: usize) -> usize { hit(2876); crate::trace::object(2876) }
unsafe extern "C" fn s2877(_a0: usize) { hit(2877);  }
unsafe extern "C" fn s2878() { hit(2878);  }
unsafe extern "C" fn s2879(_a0: usize, _a1: usize) -> usize { hit(2879); crate::trace::object(2879) }
unsafe extern "C" fn s2880(_a0: usize, _a1: usize, _a2: usize) { hit(2880);  }
unsafe extern "C" fn s2881(_a0: usize, _a1: usize) { hit(2881);  }
unsafe extern "C" fn s2882(_a0: i32) { hit(2882);  }
unsafe extern "C" fn s2883(_a0: usize, _a1: usize) { hit(2883);  }
unsafe extern "C" fn s2884(_a0: usize, _a1: usize) { hit(2884);  }
unsafe extern "C" fn s2885(_a0: usize) { hit(2885);  }
unsafe extern "C" fn s2886(_a0: usize) { hit(2886);  }
unsafe extern "C" fn s2887(_a0: usize) { hit(2887);  }
unsafe extern "C" fn s2888(_a0: i32) { hit(2888);  }
unsafe extern "C" fn s2889(_a0: usize, _a1: usize, _a2: i32) -> usize { hit(2889); crate::trace::object(2889) }
unsafe extern "C" fn s2890(_a0: usize, _a1: usize, _a2: i32) -> usize { hit(2890); crate::trace::object(2890) }
unsafe extern "C" fn s2891(_a0: usize, _a1: i32, _a2: i32) -> usize { hit(2891); crate::trace::object(2891) }
unsafe extern "C" fn s2892(_a0: usize) -> usize { hit(2892); crate::trace::object(2892) }
unsafe extern "C" fn s2893() { hit(2893);  }
unsafe extern "C" fn s2894(_a0: usize, _a1: usize, _a2: usize, _a3: i32, _a4: i32, _a5: i32, _a6: i32, _a7: f32, _a8: f32, _a9: f32, _a10: f32, _a11: i32, _a12: i32) { hit(2894);  }
unsafe extern "C" fn s2895(_a0: usize, _a1: usize, _a2: usize, _a3: i64, _a4: i64, _a5: usize, _a6: i32) -> i32 { hit(2895); crate::handles::create(2895, "SceneObject") }
unsafe extern "C" fn s2896(_a0: usize, _a1: usize) { hit(2896);  }
unsafe extern "C" fn s2897() -> i32 { hit(2897); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2898() -> i32 { hit(2898); crate::handles::create(2898, "PhysicsWorld") }
unsafe extern "C" fn s2899(_a0: usize) { hit(2899);  }
unsafe extern "C" fn s2900() -> usize { hit(2900); crate::trace::object(2900) }
unsafe extern "C" fn s2901(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize, _a6: i32, _a7: i32, _a8: f32, _a9: f32) -> i32 { hit(2901); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2902(_a0: usize) -> usize { hit(2902); crate::trace::object(2902) }
unsafe extern "C" fn s2903(_a0: usize) { hit(2903);  }
unsafe extern "C" fn s2904(_a0: usize) -> usize { hit(2904); crate::trace::object(2904) }
unsafe extern "C" fn s2905(_a0: usize) -> i32 { hit(2905); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2906(_a0: usize) -> S_RenderDeviceInfo_t { hit(2906); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2907(_a0: usize, _a1: i64) -> usize { hit(2907); crate::trace::object(2907) }
unsafe extern "C" fn s2908(_a0: usize, _a1: i32, _a2: usize, _a3: usize, _a4: i32) -> usize { hit(2908); crate::trace::object(2908) }
unsafe extern "C" fn s2909(_a0: usize, _a1: S_Color32) { hit(2909);  }
unsafe extern "C" fn s2910(_a0: usize, _a1: usize, _a2: i32, _a3: usize) { hit(2910);  }
unsafe extern "C" fn s2911(_a0: usize, _a1: i64) -> usize { hit(2911); crate::trace::object(2911) }
unsafe extern "C" fn s2912() -> usize { hit(2912); crate::trace::object(2912) }
unsafe extern "C" fn s2913(_a0: usize, _a1: usize, _a2: usize) -> i32 { hit(2913); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2914(_a0: usize) -> usize { hit(2914); crate::trace::object(2914) }
unsafe extern "C" fn s2915(_a0: usize) -> usize { hit(2915); crate::trace::object(2915) }
unsafe extern "C" fn s2916(_a0: usize) -> i64 { hit(2916); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2917(_a0: usize) -> u32 { hit(2917); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2918(_a0: u32) -> usize { hit(2918); crate::trace::object(2918) }
unsafe extern "C" fn s2919(_a0: usize) { hit(2919);  }
unsafe extern "C" fn s2920() { hit(2920);  }
unsafe extern "C" fn s2921(_a0: usize) { hit(2921);  }
unsafe extern "C" fn s2922(_a0: usize, _a1: usize, _a2: i32, _a3: i32, _a4: usize, _a5: usize, _a6: i64, _a7: i32) -> i32 { hit(2922); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2923(_a0: usize, _a1: usize, _a2: usize) -> usize { hit(2923); crate::trace::object(2923) }
unsafe extern "C" fn s2924(_a0: usize, _a1: usize) -> i32 { hit(2924); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2925(_a0: usize) { hit(2925);  }
unsafe extern "C" fn s2926() -> usize { hit(2926); crate::trace::object(2926) }
unsafe extern "C" fn s2927(_a0: usize, _a1: usize, _a2: usize) { hit(2927);  }
unsafe extern "C" fn s2928(_a0: usize) -> i32 { hit(2928); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2929() { hit(2929);  }
unsafe extern "C" fn s2930() { hit(2930);  }
unsafe extern "C" fn s2931(_a0: usize) { hit(2931);  }
unsafe extern "C" fn s2932(_a0: usize) -> i32 { hit(2932); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2933(_a0: usize, _a1: i32) { hit(2933);  }
unsafe extern "C" fn s2934() -> i32 { hit(2934); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2935(_a0: usize) -> S_Vector2 { hit(2935); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2936(_a0: usize) -> i32 { hit(2936); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2937(_a0: u32) { hit(2937);  }
unsafe extern "C" fn s2938() -> i32 { hit(2938); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2939(_a0: i32) { hit(2939);  }
unsafe extern "C" fn s2940() -> i64 { hit(2940); unsafe { crate::impls::g_pRenderDevice_GetRenderDeviceAPI() } }
unsafe extern "C" fn s2941(_a0: usize, _a1: i32) { hit(2941);  }
unsafe extern "C" fn s2942(_a0: usize) -> i32 { hit(2942); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2943() -> i32 { hit(2943); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2944() -> i32 { hit(2944); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2945(_a0: i64, _a1: usize, _a2: i64, _a3: usize) -> usize { hit(2945); crate::trace::object(2945) }
unsafe extern "C" fn s2946(_a0: usize) { hit(2946);  }
unsafe extern "C" fn s2947(_a0: usize, _a1: u32, _a2: usize, _a3: u32) -> i32 { hit(2947); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2948(_a0: i64) -> usize { hit(2948); crate::trace::object(2948) }
unsafe extern "C" fn s2949(_a0: usize) -> usize { hit(2949); crate::trace::object(2949) }
unsafe extern "C" fn s2950(_a0: usize) -> usize { hit(2950); crate::trace::object(2950) }
unsafe extern "C" fn s2951(_a0: usize, _a1: u8, _a2: i64) -> i32 { hit(2951); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2952(_a0: usize, _a1: usize, _a2: usize) { hit(2952);  }
unsafe extern "C" fn s2953(_a0: usize, _a1: usize, _a2: usize) { hit(2953);  }
unsafe extern "C" fn s2954(_a0: usize) -> S_Vector4 { hit(2954); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2955(_a0: usize) -> i32 { hit(2955); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2956(_a0: usize, _a1: i32) -> usize { hit(2956); crate::trace::object(2956) }
unsafe extern "C" fn s2957(_a0: usize, _a1: usize, _a2: i32) -> i32 { hit(2957); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2958(_a0: usize, _a1: usize) -> i32 { hit(2958); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2959(_a0: usize, _a1: usize, _a2: i32) -> usize { hit(2959); crate::trace::object(2959) }
unsafe extern "C" fn s2960(_a0: usize, _a1: usize) -> i32 { hit(2960); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2961() { hit(2961);  }
unsafe extern "C" fn s2962() { hit(2962);  }
unsafe extern "C" fn s2963(_a0: usize) -> usize { hit(2963); crate::trace::object(2963) }
unsafe extern "C" fn s2964() -> i32 { hit(2964); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2965() -> i32 { hit(2965); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2966() -> i32 { hit(2966); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2967() { hit(2967);  }
unsafe extern "C" fn s2968(_a0: usize, _a1: i32) { hit(2968);  }
unsafe extern "C" fn s2969(_a0: usize) -> usize { hit(2969); crate::trace::object(2969) }
unsafe extern "C" fn s2970(_a0: usize) { hit(2970);  }
unsafe extern "C" fn s2971(_a0: usize) { hit(2971);  }
unsafe extern "C" fn s2972(_a0: usize) -> i32 { hit(2972); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2973(_a0: usize) { hit(2973);  }
unsafe extern "C" fn s2974(_a0: usize, _a1: usize) -> i32 { hit(2974); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2975(_a0: usize) -> i32 { hit(2975); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2976() { hit(2976);  }
unsafe extern "C" fn s2977(_a0: usize) { hit(2977);  }
unsafe extern "C" fn s2978() -> i64 { hit(2978); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2979(_a0: usize) { hit(2979);  }
unsafe extern "C" fn s2980(_a0: usize) { hit(2980);  }
unsafe extern "C" fn s2981(_a0: usize, _a1: usize) -> i32 { hit(2981); crate::handles::create(2981, "SceneSkyBoxObject") }
unsafe extern "C" fn s2982(_a0: usize) -> i32 { hit(2982); crate::handles::create(2982, "DecalSceneObject") }
unsafe extern "C" fn s2983(_a0: usize) { hit(2983);  }
unsafe extern "C" fn s2984(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: i32, _a6: i64, _a7: i64, _a8: i32, _a9: f32, _a10: usize) -> usize { hit(2984); crate::trace::object(2984) }
unsafe extern "C" fn s2985(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: i32, _a6: i64, _a7: i64, _a8: i32, _a9: f32) -> usize { hit(2985); crate::trace::object(2985) }
unsafe extern "C" fn s2986(_a0: i64) -> usize { hit(2986); crate::trace::object(2986) }
unsafe extern "C" fn s2987(_a0: i64) -> usize { hit(2987); crate::trace::object(2987) }
unsafe extern "C" fn s2988() -> usize { hit(2988); crate::trace::object(2988) }
unsafe extern "C" fn s2989() -> u32 { hit(2989); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2990(_a0: usize) -> i32 { hit(2990); crate::handles::create(2990, "SceneWorld") }
unsafe extern "C" fn s2991(_a0: usize) { hit(2991);  }
unsafe extern "C" fn s2992() { hit(2992);  }
unsafe extern "C" fn s2993(_a0: i32) { hit(2993);  }
unsafe extern "C" fn s2994() { hit(2994);  }
unsafe extern "C" fn s2995() -> i32 { hit(2995); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2996() -> i32 { hit(2996); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2997() -> i32 { hit(2997); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s2998(_a0: usize, _a1: usize, _a2: usize) { hit(2998);  }
unsafe extern "C" fn s2999(_a0: usize) -> i32 { hit(2999); crate::handles::create(2999, "SceneLightObject") }
unsafe extern "C" fn s3000(_a0: usize) -> i32 { hit(3000); crate::handles::create(3000, "SceneLightObject") }
unsafe extern "C" fn s3001(_a0: usize) -> i32 { hit(3001); crate::handles::create(3001, "SceneLightObject") }
unsafe extern "C" fn s3002(_a0: usize, _a1: usize) -> i32 { hit(3002); crate::handles::create(3002, "SceneLightObject") }
unsafe extern "C" fn s3003(_a0: usize, _a1: i32) -> i32 { hit(3003); crate::handles::create(3003, "SceneLightObject") }
unsafe extern "C" fn s3004(_a0: usize) -> i32 { hit(3004); crate::handles::create(3004, "SceneLightProbeVolumeObject") }
unsafe extern "C" fn s3005(_a0: usize) { hit(3005);  }
unsafe extern "C" fn s3006(_a0: usize) { hit(3006);  }
unsafe extern "C" fn s3007(_a0: usize, _a1: i32, _a2: usize, _a3: usize, _a4: usize) -> u32 { hit(3007); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3008(_a0: usize, _a1: u32) { hit(3008);  }
unsafe extern "C" fn s3009(_a0: usize, _a1: usize) -> u32 { hit(3009); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3010(_a0: usize, _a1: u32) { hit(3010);  }
unsafe extern "C" fn s3011(_a0: usize, _a1: usize, _a2: u8) { hit(3011);  }
unsafe extern "C" fn s3012(_a0: usize, _a1: i32, _a2: i32) { hit(3012);  }
unsafe extern "C" fn s3013(_a0: i32, _a1: i32) { hit(3013);  }
unsafe extern "C" fn s3014(_a0: usize) { hit(3014);  }
unsafe extern "C" fn s3015(_a0: usize, _a1: i32) -> usize { hit(3015); crate::trace::object(3015) }
unsafe extern "C" fn s3016(_a0: usize) { hit(3016);  }
unsafe extern "C" fn s3017(_a0: i64) { hit(3017);  }
unsafe extern "C" fn s3018() -> i64 { hit(3018); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3019() { hit(3019);  }
unsafe extern "C" fn s3020() -> i32 { hit(3020); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3021() -> i32 { hit(3021); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3022() -> u64 { hit(3022); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3023(_a0: i32) -> f32 { hit(3023); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3024(_a0: i32) -> usize { hit(3024); crate::trace::object(3024) }
unsafe extern "C" fn s3025(_a0: i32) -> i32 { hit(3025); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3026(_a0: i32) -> u32 { hit(3026); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3027(_a0: i32) -> i32 { hit(3027); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3028(_a0: i32) -> i32 { hit(3028); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3029() -> i32 { hit(3029); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3030(_a0: i32, _a1: usize) -> usize { hit(3030); crate::trace::object(3030) }
unsafe extern "C" fn s3031(_a0: usize, _a1: usize, _a2: i32, _a3: i32) -> usize { hit(3031); crate::trace::object(3031) }
unsafe extern "C" fn s3032(_a0: usize, _a1: usize) { hit(3032);  }
unsafe extern "C" fn s3033(_a0: f32) { hit(3033);  }
unsafe extern "C" fn s3034() -> f32 { hit(3034); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3035() -> usize { hit(3035); crate::trace::object(3035) }
unsafe extern "C" fn s3036(_a0: usize) { hit(3036);  }
unsafe extern "C" fn s3037() -> usize { hit(3037); crate::trace::object(3037) }
unsafe extern "C" fn s3038(_a0: usize) { hit(3038);  }
unsafe extern "C" fn s3039() -> i32 { hit(3039); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3040(_a0: i32) -> S_NativeEngine_AudioDeviceDesc { hit(3040); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3041(_a0: i32) -> usize { hit(3041); crate::trace::object(3041) }
unsafe extern "C" fn s3042(_a0: i32) -> usize { hit(3042); crate::trace::object(3042) }
unsafe extern "C" fn s3043() -> usize { hit(3043); crate::trace::object(3043) }
unsafe extern "C" fn s3044(_a0: usize) { hit(3044);  }
unsafe extern "C" fn s3045(_a0: usize) { hit(3045);  }
unsafe extern "C" fn s3046(_a0: usize) -> usize { hit(3046); crate::trace::object(3046) }
unsafe extern "C" fn s3047(_a0: usize, _a1: i32, _a2: i32, _a3: i32, _a4: i32, _a5: f32, _a6: i32, _a7: i32, _a8: usize, _a9: i32) -> usize { hit(3047); crate::trace::object(3047) }
unsafe extern "C" fn s3048(_a0: usize) { hit(3048);  }
unsafe extern "C" fn s3049() { hit(3049);  }
unsafe extern "C" fn s3050(_a0: S_Sandbox_StringToken, _a1: usize, _a2: f32, _a3: f32) -> u32 { hit(3050); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3051(_a0: usize, _a1: usize, _a2: i32, _a3: i32, _a4: i32, _a5: i32, _a6: S_Sandbox_StringToken, _a7: usize) -> usize { hit(3051); crate::trace::object(3051) }
unsafe extern "C" fn s3052(_a0: usize, _a1: usize) -> i32 { hit(3052); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3053(_a0: usize) -> i32 { hit(3053); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3054(_a0: usize, _a1: usize, _a2: usize) { hit(3054);  }
unsafe extern "C" fn s3055(_a0: usize, _a1: usize, _a2: usize) { hit(3055);  }
unsafe extern "C" fn s3056(_a0: usize, _a1: usize) { hit(3056); unsafe { crate::impls::global_Plat_MessageBox(_a0, _a1) } }
unsafe extern "C" fn s3057(_a0: usize) -> i32 { hit(3057); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3058(_a0: usize) { hit(3058);  }
unsafe extern "C" fn s3059(_a0: usize) { hit(3059); unsafe { crate::impls::global_Plat_SetCurrentDirectory(_a0) } }
unsafe extern "C" fn s3060() -> u64 { hit(3060); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3061(_a0: u64) { hit(3061);  }
unsafe extern "C" fn s3062(_a0: i64) { hit(3062);  }
unsafe extern "C" fn s3063() -> i32 { hit(3063); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3064() -> i32 { hit(3064); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3065(_a0: usize) -> i32 { hit(3065); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3066() { hit(3066);  }
unsafe extern "C" fn s3067(_a0: i32) { hit(3067); unsafe { crate::impls::global_Plat_ExitProcess(_a0) } }
unsafe extern "C" fn s3068() -> usize { hit(3068); unsafe { crate::impls::global_GetGameRootFolder() } }
unsafe extern "C" fn s3069() -> usize { hit(3069); crate::trace::object(3069) }
unsafe extern "C" fn s3070() -> i32 { hit(3070); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3071(_a0: usize) -> i32 { hit(3071); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3072(_a0: usize, _a1: usize) -> i32 { hit(3072); unsafe { crate::impls::global_SourceEnginePreInit(_a0, _a1) } }
unsafe extern "C" fn s3073(_a0: usize) -> i32 { hit(3073); unsafe { crate::impls::global_SourceEngineInit(_a0) } }
unsafe extern "C" fn s3074(_a0: usize, _a1: i32) -> i32 { hit(3074); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3075() { hit(3075);  }
unsafe extern "C" fn s3076(_a0: usize, _a1: f64, _a2: f64) -> i32 { hit(3076); unsafe { crate::impls::global_SourceEngineFrame(_a0, _a1, _a2) } }
unsafe extern "C" fn s3077(_a0: usize, _a1: i32) { hit(3077);  }
unsafe extern "C" fn s3078() -> i32 { hit(3078); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3079() { hit(3079);  }
unsafe extern "C" fn s3080(_a0: usize) -> S_b2WorldId { hit(3080); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3081(_a0: S_b2WorldId, _a1: usize) -> S_b2BodyId { hit(3081); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3082(_a0: S_b2BodyId, _a1: usize, _a2: usize) -> S_b2ShapeId { hit(3082); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3083(_a0: S_b2BodyId, _a1: usize, _a2: usize) -> S_b2ShapeId { hit(3083); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3084(_a0: S_b2BodyId, _a1: usize, _a2: usize) -> S_b2ShapeId { hit(3084); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3085(_a0: S_b2BodyId, _a1: usize, _a2: usize) -> S_b2ShapeId { hit(3085); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3086(_a0: S_b2BodyId, _a1: usize) -> S_b2ChainId { hit(3086); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3087(_a0: S_b2WorldId, _a1: usize) -> S_b2JointId { hit(3087); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3088(_a0: S_b2WorldId, _a1: usize) -> S_b2JointId { hit(3088); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3089(_a0: S_b2WorldId, _a1: usize) -> S_b2JointId { hit(3089); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3090(_a0: S_b2WorldId, _a1: usize) -> S_b2JointId { hit(3090); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3091(_a0: S_b2WorldId, _a1: usize) -> S_b2JointId { hit(3091); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3092(_a0: S_b2WorldId, _a1: usize) -> S_b2JointId { hit(3092); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3093(_a0: S_b2WorldId, _a1: usize) -> S_b2JointId { hit(3093); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3094(_a0: S_b2WorldId, _a1: usize) { hit(3094);  }
unsafe extern "C" fn s3095(_a0: S_b2WorldId) { hit(3095);  }
unsafe extern "C" fn s3096(_a0: f32) { hit(3096);  }
unsafe extern "C" fn s3097() -> f32 { hit(3097); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3098(_a0: S_b2WorldId) -> i32 { hit(3098); 1 }
unsafe extern "C" fn s3099(_a0: S_b2WorldId, _a1: f32, _a2: i32) { hit(3099);  }
unsafe extern "C" fn s3100(_a0: S_b2WorldId, _a1: usize) { hit(3100);  }
unsafe extern "C" fn s3101(_a0: S_b2WorldId) -> S_b2BodyEvents { hit(3101); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3102(_a0: S_b2WorldId) -> S_b2SensorEvents { hit(3102); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3103(_a0: S_b2WorldId) -> S_b2ContactEvents { hit(3103); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3104(_a0: S_b2WorldId) -> S_b2JointEvents { hit(3104); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3105(_a0: S_b2WorldId, _a1: usize, _a2: usize, _a3: usize, _a4: usize) -> S_b2TreeStats { hit(3105); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3106(_a0: S_b2WorldId, _a1: usize, _a2: usize, _a3: usize, _a4: usize) -> S_b2TreeStats { hit(3106); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3107(_a0: S_b2WorldId, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize) -> S_b2TreeStats { hit(3107); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3108(_a0: S_b2WorldId, _a1: usize, _a2: usize, _a3: usize) -> S_b2RayResult { hit(3108); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3109(_a0: S_b2WorldId, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize) -> S_b2TreeStats { hit(3109); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3110(_a0: S_b2WorldId, _a1: usize, _a2: usize, _a3: usize) -> f32 { hit(3110); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3111(_a0: S_b2WorldId, _a1: usize, _a2: usize, _a3: usize, _a4: usize) { hit(3111);  }
unsafe extern "C" fn s3112(_a0: S_b2WorldId, _a1: i32) { hit(3112);  }
unsafe extern "C" fn s3113(_a0: S_b2WorldId) -> i32 { hit(3113); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3114(_a0: S_b2WorldId, _a1: i32) { hit(3114);  }
unsafe extern "C" fn s3115(_a0: S_b2WorldId) -> i32 { hit(3115); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3116(_a0: S_b2WorldId, _a1: f32) { hit(3116);  }
unsafe extern "C" fn s3117(_a0: S_b2WorldId) -> f32 { hit(3117); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3118(_a0: S_b2WorldId, _a1: f32) { hit(3118);  }
unsafe extern "C" fn s3119(_a0: S_b2WorldId) -> f32 { hit(3119); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3120(_a0: S_b2WorldId, _a1: usize, _a2: usize) { hit(3120);  }
unsafe extern "C" fn s3121(_a0: S_b2WorldId, _a1: usize, _a2: usize) { hit(3121);  }
unsafe extern "C" fn s3122(_a0: S_b2WorldId, _a1: usize) { hit(3122);  }
unsafe extern "C" fn s3123(_a0: S_b2WorldId) -> S_b2Vec2 { hit(3123); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3124(_a0: S_b2WorldId, _a1: f32, _a2: f32, _a3: f32) { hit(3124);  }
unsafe extern "C" fn s3125(_a0: S_b2WorldId, _a1: f32) { hit(3125);  }
unsafe extern "C" fn s3126(_a0: S_b2WorldId) -> f32 { hit(3126); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3127(_a0: S_b2WorldId, _a1: f32) { hit(3127);  }
unsafe extern "C" fn s3128(_a0: S_b2WorldId) -> f32 { hit(3128); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3129(_a0: S_b2WorldId, _a1: i32) { hit(3129);  }
unsafe extern "C" fn s3130(_a0: S_b2WorldId) -> i32 { hit(3130); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3131(_a0: S_b2WorldId) -> i32 { hit(3131); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3132(_a0: S_b2WorldId) -> S_b2Profile { hit(3132); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3133(_a0: S_b2WorldId) -> S_b2Counters { hit(3133); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3134(_a0: S_b2WorldId, _a1: usize) { hit(3134);  }
unsafe extern "C" fn s3135(_a0: S_b2WorldId) -> usize { hit(3135); crate::trace::object(3135) }
unsafe extern "C" fn s3136(_a0: S_b2WorldId, _a1: usize) { hit(3136);  }
unsafe extern "C" fn s3137(_a0: S_b2WorldId, _a1: usize) { hit(3137);  }
unsafe extern "C" fn s3138(_a0: S_b2WorldId) { hit(3138);  }
unsafe extern "C" fn s3139(_a0: S_b2WorldId) { hit(3139);  }
unsafe extern "C" fn s3140(_a0: S_b2WorldId, _a1: i32) { hit(3140);  }
unsafe extern "C" fn s3141(_a0: S_b2WorldId) -> S_b2Capacity { hit(3141); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3142(_a0: S_b2WorldId) -> i32 { hit(3142); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3143(_a0: S_b2WorldId, _a1: i32) { hit(3143);  }
unsafe extern "C" fn s3144(_a0: S_b2BodyId) { hit(3144);  }
unsafe extern "C" fn s3145(_a0: S_b2BodyId) -> i32 { hit(3145); 1 }
unsafe extern "C" fn s3146(_a0: S_b2BodyId) -> i64 { hit(3146); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3147(_a0: S_b2BodyId, _a1: i64) { hit(3147);  }
unsafe extern "C" fn s3148(_a0: S_b2BodyId, _a1: usize) { hit(3148);  }
unsafe extern "C" fn s3149(_a0: S_b2BodyId) -> usize { hit(3149); crate::trace::object(3149) }
unsafe extern "C" fn s3150(_a0: S_b2BodyId, _a1: usize) { hit(3150);  }
unsafe extern "C" fn s3151(_a0: S_b2BodyId) -> usize { hit(3151); crate::trace::object(3151) }
unsafe extern "C" fn s3152(_a0: S_b2BodyId) -> S_b2Vec2 { hit(3152); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3153(_a0: S_b2BodyId) -> S_b2Rot { hit(3153); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3154(_a0: S_b2BodyId) -> S_b2Transform { hit(3154); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3155(_a0: S_b2BodyId, _a1: usize, _a2: usize) { hit(3155);  }
unsafe extern "C" fn s3156(_a0: S_b2BodyId, _a1: usize) -> S_b2Vec2 { hit(3156); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3157(_a0: S_b2BodyId, _a1: usize) -> S_b2Vec2 { hit(3157); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3158(_a0: S_b2BodyId, _a1: usize) -> S_b2Vec2 { hit(3158); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3159(_a0: S_b2BodyId, _a1: usize) -> S_b2Vec2 { hit(3159); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3160(_a0: S_b2BodyId) -> S_b2Vec2 { hit(3160); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3161(_a0: S_b2BodyId) -> f32 { hit(3161); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3162(_a0: S_b2BodyId, _a1: usize) { hit(3162);  }
unsafe extern "C" fn s3163(_a0: S_b2BodyId, _a1: f32) { hit(3163);  }
unsafe extern "C" fn s3164(_a0: S_b2BodyId, _a1: usize, _a2: f32, _a3: i32) { hit(3164);  }
unsafe extern "C" fn s3165(_a0: S_b2BodyId, _a1: usize) -> S_b2Vec2 { hit(3165); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3166(_a0: S_b2BodyId, _a1: usize) -> S_b2Vec2 { hit(3166); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3167(_a0: S_b2BodyId, _a1: usize, _a2: usize, _a3: i32) { hit(3167);  }
unsafe extern "C" fn s3168(_a0: S_b2BodyId, _a1: usize, _a2: i32) { hit(3168);  }
unsafe extern "C" fn s3169(_a0: S_b2BodyId, _a1: f32, _a2: i32) { hit(3169);  }
unsafe extern "C" fn s3170(_a0: S_b2BodyId) { hit(3170);  }
unsafe extern "C" fn s3171(_a0: S_b2BodyId, _a1: usize, _a2: usize, _a3: i32) { hit(3171);  }
unsafe extern "C" fn s3172(_a0: S_b2BodyId, _a1: usize, _a2: i32) { hit(3172);  }
unsafe extern "C" fn s3173(_a0: S_b2BodyId, _a1: f32, _a2: i32) { hit(3173);  }
unsafe extern "C" fn s3174(_a0: S_b2BodyId) -> f32 { hit(3174); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3175(_a0: S_b2BodyId) -> f32 { hit(3175); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3176(_a0: S_b2BodyId) -> S_b2Vec2 { hit(3176); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3177(_a0: S_b2BodyId) -> S_b2Vec2 { hit(3177); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3178(_a0: S_b2BodyId, _a1: usize) { hit(3178);  }
unsafe extern "C" fn s3179(_a0: S_b2BodyId) -> S_b2MassData { hit(3179); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3180(_a0: S_b2BodyId) { hit(3180);  }
unsafe extern "C" fn s3181(_a0: S_b2BodyId, _a1: f32) { hit(3181);  }
unsafe extern "C" fn s3182(_a0: S_b2BodyId) -> f32 { hit(3182); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3183(_a0: S_b2BodyId, _a1: f32) { hit(3183);  }
unsafe extern "C" fn s3184(_a0: S_b2BodyId) -> f32 { hit(3184); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3185(_a0: S_b2BodyId, _a1: f32) { hit(3185);  }
unsafe extern "C" fn s3186(_a0: S_b2BodyId) -> f32 { hit(3186); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3187(_a0: S_b2BodyId) -> i32 { hit(3187); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3188(_a0: S_b2BodyId, _a1: i32) { hit(3188);  }
unsafe extern "C" fn s3189(_a0: S_b2BodyId) { hit(3189);  }
unsafe extern "C" fn s3190(_a0: S_b2BodyId, _a1: i32) { hit(3190);  }
unsafe extern "C" fn s3191(_a0: S_b2BodyId) -> i32 { hit(3191); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3192(_a0: S_b2BodyId, _a1: f32) { hit(3192);  }
unsafe extern "C" fn s3193(_a0: S_b2BodyId) -> f32 { hit(3193); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3194(_a0: S_b2BodyId) -> i32 { hit(3194); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3195(_a0: S_b2BodyId) { hit(3195);  }
unsafe extern "C" fn s3196(_a0: S_b2BodyId) { hit(3196);  }
unsafe extern "C" fn s3197(_a0: S_b2BodyId, _a1: S_b2MotionLocks) { hit(3197);  }
unsafe extern "C" fn s3198(_a0: S_b2BodyId) -> S_b2MotionLocks { hit(3198); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3199(_a0: S_b2BodyId, _a1: i32) { hit(3199);  }
unsafe extern "C" fn s3200(_a0: S_b2BodyId) -> i32 { hit(3200); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3201(_a0: S_b2BodyId, _a1: i32) { hit(3201);  }
unsafe extern "C" fn s3202(_a0: S_b2BodyId, _a1: i32) { hit(3202);  }
unsafe extern "C" fn s3203(_a0: S_b2BodyId) -> S_b2WorldId { hit(3203); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3204(_a0: S_b2BodyId) -> i32 { hit(3204); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3205(_a0: S_b2BodyId, _a1: usize, _a2: i32) -> i32 { hit(3205); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3206(_a0: S_b2BodyId) -> i32 { hit(3206); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3207(_a0: S_b2BodyId, _a1: usize, _a2: i32) -> i32 { hit(3207); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3208(_a0: S_b2BodyId) -> i32 { hit(3208); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3209(_a0: S_b2BodyId, _a1: usize, _a2: i32) -> i32 { hit(3209); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3210(_a0: S_b2BodyId) -> S_b2AABB { hit(3210); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3211(_a0: S_b2ShapeId, _a1: i32) { hit(3211);  }
unsafe extern "C" fn s3212(_a0: S_b2ShapeId) -> i32 { hit(3212); 1 }
unsafe extern "C" fn s3213(_a0: S_b2ShapeId) -> i64 { hit(3213); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3214(_a0: S_b2ShapeId) -> S_b2BodyId { hit(3214); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3215(_a0: S_b2ShapeId) -> S_b2WorldId { hit(3215); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3216(_a0: S_b2ShapeId) -> i32 { hit(3216); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3217(_a0: S_b2ShapeId, _a1: usize) { hit(3217);  }
unsafe extern "C" fn s3218(_a0: S_b2ShapeId) -> usize { hit(3218); crate::trace::object(3218) }
unsafe extern "C" fn s3219(_a0: S_b2ShapeId, _a1: f32, _a2: i32) { hit(3219);  }
unsafe extern "C" fn s3220(_a0: S_b2ShapeId) -> f32 { hit(3220); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3221(_a0: S_b2ShapeId, _a1: f32) { hit(3221);  }
unsafe extern "C" fn s3222(_a0: S_b2ShapeId) -> f32 { hit(3222); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3223(_a0: S_b2ShapeId, _a1: f32) { hit(3223);  }
unsafe extern "C" fn s3224(_a0: S_b2ShapeId) -> f32 { hit(3224); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3225(_a0: S_b2ShapeId, _a1: u64) { hit(3225);  }
unsafe extern "C" fn s3226(_a0: S_b2ShapeId) -> u64 { hit(3226); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3227(_a0: S_b2ShapeId, _a1: usize) { hit(3227);  }
unsafe extern "C" fn s3228(_a0: S_b2ShapeId) -> S_b2SurfaceMaterial { hit(3228); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3229(_a0: S_b2ShapeId) -> S_b2Filter { hit(3229); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3230(_a0: S_b2ShapeId, _a1: usize) { hit(3230);  }
unsafe extern "C" fn s3231(_a0: S_b2ShapeId, _a1: i32) { hit(3231);  }
unsafe extern "C" fn s3232(_a0: S_b2ShapeId) -> i32 { hit(3232); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3233(_a0: S_b2ShapeId, _a1: i32) { hit(3233);  }
unsafe extern "C" fn s3234(_a0: S_b2ShapeId) -> i32 { hit(3234); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3235(_a0: S_b2ShapeId, _a1: i32) { hit(3235);  }
unsafe extern "C" fn s3236(_a0: S_b2ShapeId) -> i32 { hit(3236); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3237(_a0: S_b2ShapeId, _a1: i32) { hit(3237);  }
unsafe extern "C" fn s3238(_a0: S_b2ShapeId) -> i32 { hit(3238); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3239(_a0: S_b2ShapeId, _a1: usize) -> i32 { hit(3239); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3240(_a0: S_b2ShapeId, _a1: usize) -> S_b2CastOutput { hit(3240); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3241(_a0: S_b2ShapeId) -> S_b2Circle { hit(3241); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3242(_a0: S_b2ShapeId) -> S_b2Segment { hit(3242); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3243(_a0: S_b2ShapeId) -> S_b2ChainSegment { hit(3243); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3244(_a0: S_b2ShapeId) -> S_b2Capsule { hit(3244); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3245(_a0: S_b2ShapeId) -> S_b2Polygon { hit(3245); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3246(_a0: S_b2ShapeId, _a1: usize) { hit(3246);  }
unsafe extern "C" fn s3247(_a0: S_b2ShapeId, _a1: usize) { hit(3247);  }
unsafe extern "C" fn s3248(_a0: S_b2ShapeId, _a1: usize) { hit(3248);  }
unsafe extern "C" fn s3249(_a0: S_b2ShapeId, _a1: usize) { hit(3249);  }
unsafe extern "C" fn s3250(_a0: S_b2ShapeId) -> S_b2ChainId { hit(3250); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3251(_a0: S_b2ShapeId) -> i32 { hit(3251); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3252(_a0: S_b2ShapeId, _a1: usize, _a2: i32) -> i32 { hit(3252); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3253(_a0: S_b2ShapeId) -> i32 { hit(3253); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3254(_a0: S_b2ShapeId, _a1: usize, _a2: i32) -> i32 { hit(3254); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3255(_a0: S_b2ShapeId) -> S_b2AABB { hit(3255); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3256(_a0: S_b2ShapeId) -> S_b2MassData { hit(3256); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3257(_a0: S_b2ShapeId, _a1: usize) -> S_b2Vec2 { hit(3257); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3258(_a0: S_b2ShapeId, _a1: usize, _a2: f32, _a3: f32, _a4: i32) { hit(3258);  }
unsafe extern "C" fn s3259(_a0: S_b2ChainId) { hit(3259);  }
unsafe extern "C" fn s3260(_a0: S_b2ChainId) -> S_b2WorldId { hit(3260); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3261(_a0: S_b2ChainId) -> i32 { hit(3261); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3262(_a0: S_b2ChainId, _a1: usize, _a2: i32) -> i32 { hit(3262); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3263(_a0: S_b2ChainId) -> i32 { hit(3263); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3264(_a0: S_b2ChainId, _a1: usize, _a2: i32) { hit(3264);  }
unsafe extern "C" fn s3265(_a0: S_b2ChainId, _a1: i32) -> S_b2SurfaceMaterial { hit(3265); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3266(_a0: S_b2ChainId) -> i32 { hit(3266); 1 }
unsafe extern "C" fn s3267(_a0: S_b2JointId, _a1: i32) { hit(3267);  }
unsafe extern "C" fn s3268(_a0: S_b2JointId) -> i32 { hit(3268); 1 }
unsafe extern "C" fn s3269(_a0: S_b2JointId) -> i64 { hit(3269); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3270(_a0: S_b2JointId) -> S_b2BodyId { hit(3270); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3271(_a0: S_b2JointId) -> S_b2BodyId { hit(3271); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3272(_a0: S_b2JointId) -> S_b2WorldId { hit(3272); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3273(_a0: S_b2JointId, _a1: usize) { hit(3273);  }
unsafe extern "C" fn s3274(_a0: S_b2JointId) -> S_b2Transform { hit(3274); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3275(_a0: S_b2JointId, _a1: usize) { hit(3275);  }
unsafe extern "C" fn s3276(_a0: S_b2JointId) -> S_b2Transform { hit(3276); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3277(_a0: S_b2JointId, _a1: i32) { hit(3277);  }
unsafe extern "C" fn s3278(_a0: S_b2JointId) -> i32 { hit(3278); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3279(_a0: S_b2JointId, _a1: usize) { hit(3279);  }
unsafe extern "C" fn s3280(_a0: S_b2JointId) -> usize { hit(3280); crate::trace::object(3280) }
unsafe extern "C" fn s3281(_a0: S_b2JointId) { hit(3281);  }
unsafe extern "C" fn s3282(_a0: S_b2JointId) -> S_b2Vec2 { hit(3282); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3283(_a0: S_b2JointId) -> f32 { hit(3283); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3284(_a0: S_b2JointId) -> f32 { hit(3284); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3285(_a0: S_b2JointId) -> f32 { hit(3285); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3286(_a0: S_b2JointId, _a1: f32, _a2: f32) { hit(3286);  }
unsafe extern "C" fn s3287(_a0: S_b2JointId, _a1: usize, _a2: usize) { hit(3287);  }
unsafe extern "C" fn s3288(_a0: S_b2JointId, _a1: f32) { hit(3288);  }
unsafe extern "C" fn s3289(_a0: S_b2JointId) -> f32 { hit(3289); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3290(_a0: S_b2JointId, _a1: f32) { hit(3290);  }
unsafe extern "C" fn s3291(_a0: S_b2JointId) -> f32 { hit(3291); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3292(_a0: S_b2JointId, _a1: f32) { hit(3292);  }
unsafe extern "C" fn s3293(_a0: S_b2JointId) -> f32 { hit(3293); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3294(_a0: S_b2JointId, _a1: i32) { hit(3294);  }
unsafe extern "C" fn s3295(_a0: S_b2JointId) -> i32 { hit(3295); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3296(_a0: S_b2JointId, _a1: f32, _a2: f32) { hit(3296);  }
unsafe extern "C" fn s3297(_a0: S_b2JointId, _a1: usize, _a2: usize) { hit(3297);  }
unsafe extern "C" fn s3298(_a0: S_b2JointId, _a1: f32) { hit(3298);  }
unsafe extern "C" fn s3299(_a0: S_b2JointId, _a1: f32) { hit(3299);  }
unsafe extern "C" fn s3300(_a0: S_b2JointId) -> f32 { hit(3300); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3301(_a0: S_b2JointId) -> f32 { hit(3301); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3302(_a0: S_b2JointId, _a1: i32) { hit(3302);  }
unsafe extern "C" fn s3303(_a0: S_b2JointId) -> i32 { hit(3303); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3304(_a0: S_b2JointId, _a1: f32, _a2: f32) { hit(3304);  }
unsafe extern "C" fn s3305(_a0: S_b2JointId) -> f32 { hit(3305); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3306(_a0: S_b2JointId) -> f32 { hit(3306); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3307(_a0: S_b2JointId) -> f32 { hit(3307); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3308(_a0: S_b2JointId, _a1: i32) { hit(3308);  }
unsafe extern "C" fn s3309(_a0: S_b2JointId) -> i32 { hit(3309); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3310(_a0: S_b2JointId, _a1: f32) { hit(3310);  }
unsafe extern "C" fn s3311(_a0: S_b2JointId) -> f32 { hit(3311); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3312(_a0: S_b2JointId, _a1: f32) { hit(3312);  }
unsafe extern "C" fn s3313(_a0: S_b2JointId) -> f32 { hit(3313); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3314(_a0: S_b2JointId) -> f32 { hit(3314); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3315(_a0: S_b2JointId, _a1: usize) { hit(3315);  }
unsafe extern "C" fn s3316(_a0: S_b2JointId) -> S_b2Vec2 { hit(3316); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3317(_a0: S_b2JointId, _a1: f32) { hit(3317);  }
unsafe extern "C" fn s3318(_a0: S_b2JointId) -> f32 { hit(3318); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3319(_a0: S_b2JointId, _a1: f32) { hit(3319);  }
unsafe extern "C" fn s3320(_a0: S_b2JointId) -> f32 { hit(3320); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3321(_a0: S_b2JointId, _a1: f32) { hit(3321);  }
unsafe extern "C" fn s3322(_a0: S_b2JointId) -> f32 { hit(3322); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3323(_a0: S_b2JointId, _a1: f32) { hit(3323);  }
unsafe extern "C" fn s3324(_a0: S_b2JointId) -> f32 { hit(3324); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3325(_a0: S_b2JointId, _a1: f32) { hit(3325);  }
unsafe extern "C" fn s3326(_a0: S_b2JointId) -> f32 { hit(3326); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3327(_a0: S_b2JointId, _a1: f32) { hit(3327);  }
unsafe extern "C" fn s3328(_a0: S_b2JointId) -> f32 { hit(3328); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3329(_a0: S_b2JointId, _a1: f32) { hit(3329);  }
unsafe extern "C" fn s3330(_a0: S_b2JointId) -> f32 { hit(3330); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3331(_a0: S_b2JointId, _a1: f32) { hit(3331);  }
unsafe extern "C" fn s3332(_a0: S_b2JointId) -> f32 { hit(3332); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3333(_a0: S_b2JointId, _a1: f32) { hit(3333);  }
unsafe extern "C" fn s3334(_a0: S_b2JointId) -> f32 { hit(3334); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3335(_a0: S_b2JointId, _a1: i32) { hit(3335);  }
unsafe extern "C" fn s3336(_a0: S_b2JointId) -> i32 { hit(3336); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3337(_a0: S_b2JointId, _a1: f32) { hit(3337);  }
unsafe extern "C" fn s3338(_a0: S_b2JointId) -> f32 { hit(3338); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3339(_a0: S_b2JointId, _a1: f32) { hit(3339);  }
unsafe extern "C" fn s3340(_a0: S_b2JointId) -> f32 { hit(3340); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3341(_a0: S_b2JointId, _a1: f32) { hit(3341);  }
unsafe extern "C" fn s3342(_a0: S_b2JointId) -> f32 { hit(3342); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3343(_a0: S_b2JointId, _a1: i32) { hit(3343);  }
unsafe extern "C" fn s3344(_a0: S_b2JointId) -> i32 { hit(3344); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3345(_a0: S_b2JointId) -> f32 { hit(3345); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3346(_a0: S_b2JointId) -> f32 { hit(3346); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3347(_a0: S_b2JointId, _a1: f32, _a2: f32) { hit(3347);  }
unsafe extern "C" fn s3348(_a0: S_b2JointId, _a1: i32) { hit(3348);  }
unsafe extern "C" fn s3349(_a0: S_b2JointId) -> i32 { hit(3349); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3350(_a0: S_b2JointId, _a1: f32) { hit(3350);  }
unsafe extern "C" fn s3351(_a0: S_b2JointId) -> f32 { hit(3351); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3352(_a0: S_b2JointId, _a1: f32) { hit(3352);  }
unsafe extern "C" fn s3353(_a0: S_b2JointId) -> f32 { hit(3353); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3354(_a0: S_b2JointId) -> f32 { hit(3354); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3355(_a0: S_b2JointId) -> f32 { hit(3355); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3356(_a0: S_b2JointId) -> f32 { hit(3356); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3357(_a0: S_b2JointId, _a1: i32) { hit(3357);  }
unsafe extern "C" fn s3358(_a0: S_b2JointId) -> i32 { hit(3358); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3359(_a0: S_b2JointId, _a1: f32) { hit(3359);  }
unsafe extern "C" fn s3360(_a0: S_b2JointId) -> f32 { hit(3360); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3361(_a0: S_b2JointId, _a1: f32) { hit(3361);  }
unsafe extern "C" fn s3362(_a0: S_b2JointId) -> f32 { hit(3362); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3363(_a0: S_b2JointId, _a1: f32) { hit(3363);  }
unsafe extern "C" fn s3364(_a0: S_b2JointId) -> f32 { hit(3364); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3365(_a0: S_b2JointId) -> f32 { hit(3365); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3366(_a0: S_b2JointId, _a1: i32) { hit(3366);  }
unsafe extern "C" fn s3367(_a0: S_b2JointId) -> i32 { hit(3367); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3368(_a0: S_b2JointId) -> f32 { hit(3368); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3369(_a0: S_b2JointId) -> f32 { hit(3369); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3370(_a0: S_b2JointId, _a1: f32, _a2: f32) { hit(3370);  }
unsafe extern "C" fn s3371(_a0: S_b2JointId, _a1: i32) { hit(3371);  }
unsafe extern "C" fn s3372(_a0: S_b2JointId) -> i32 { hit(3372); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3373(_a0: S_b2JointId, _a1: f32) { hit(3373);  }
unsafe extern "C" fn s3374(_a0: S_b2JointId) -> f32 { hit(3374); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3375(_a0: S_b2JointId) -> f32 { hit(3375); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3376(_a0: S_b2JointId, _a1: f32) { hit(3376);  }
unsafe extern "C" fn s3377(_a0: S_b2JointId) -> f32 { hit(3377); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3378(_a0: S_b2JointId, _a1: f32) { hit(3378);  }
unsafe extern "C" fn s3379(_a0: S_b2JointId) -> f32 { hit(3379); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3380(_a0: S_b2JointId, _a1: f32) { hit(3380);  }
unsafe extern "C" fn s3381(_a0: S_b2JointId) -> f32 { hit(3381); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3382(_a0: S_b2JointId, _a1: f32) { hit(3382);  }
unsafe extern "C" fn s3383(_a0: S_b2JointId) -> f32 { hit(3383); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3384(_a0: S_b2JointId, _a1: f32) { hit(3384);  }
unsafe extern "C" fn s3385(_a0: S_b2JointId) -> f32 { hit(3385); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3386(_a0: S_b2JointId, _a1: i32) { hit(3386);  }
unsafe extern "C" fn s3387(_a0: S_b2JointId) -> i32 { hit(3387); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3388(_a0: S_b2JointId, _a1: f32) { hit(3388);  }
unsafe extern "C" fn s3389(_a0: S_b2JointId) -> f32 { hit(3389); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3390(_a0: S_b2JointId, _a1: f32) { hit(3390);  }
unsafe extern "C" fn s3391(_a0: S_b2JointId) -> f32 { hit(3391); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3392(_a0: S_b2JointId, _a1: i32) { hit(3392);  }
unsafe extern "C" fn s3393(_a0: S_b2JointId) -> i32 { hit(3393); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3394(_a0: S_b2JointId) -> f32 { hit(3394); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3395(_a0: S_b2JointId) -> f32 { hit(3395); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3396(_a0: S_b2JointId, _a1: f32, _a2: f32) { hit(3396);  }
unsafe extern "C" fn s3397(_a0: S_b2JointId, _a1: i32) { hit(3397);  }
unsafe extern "C" fn s3398(_a0: S_b2JointId) -> i32 { hit(3398); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3399(_a0: S_b2JointId, _a1: f32) { hit(3399);  }
unsafe extern "C" fn s3400(_a0: S_b2JointId) -> f32 { hit(3400); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3401(_a0: S_b2JointId, _a1: f32) { hit(3401);  }
unsafe extern "C" fn s3402(_a0: S_b2JointId) -> f32 { hit(3402); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3403(_a0: S_b2JointId) -> f32 { hit(3403); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3404(_a0: S_b2ContactId) -> i32 { hit(3404); 1 }
unsafe extern "C" fn s3405(_a0: S_b2ContactId) -> S_b2ContactData { hit(3405); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3406() -> S_b2WorldDef { hit(3406); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3407() -> S_b2BodyDef { hit(3407); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3408() -> S_b2ShapeDef { hit(3408); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3409() -> S_b2ChainDef { hit(3409); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3410() -> S_b2DistanceJointDef { hit(3410); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3411() -> S_b2MotorJointDef { hit(3411); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3412() -> S_b2FilterJointDef { hit(3412); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3413() -> S_b2PrismaticJointDef { hit(3413); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3414() -> S_b2RevoluteJointDef { hit(3414); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3415() -> S_b2WeldJointDef { hit(3415); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3416() -> S_b2WheelJointDef { hit(3416); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3417() -> S_b2ExplosionDef { hit(3417); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3418() -> S_b2Filter { hit(3418); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3419() -> S_b2QueryFilter { hit(3419); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3420() -> S_b2SurfaceMaterial { hit(3420); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3421(_a0: f32, _a1: f32) -> S_b2Polygon { hit(3421); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3422(_a0: f32, _a1: f32, _a2: usize, _a3: usize) -> S_b2Polygon { hit(3422); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3423(_a0: usize, _a1: i32) -> S_b2Hull { hit(3423); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3424(_a0: usize, _a1: f32) -> S_b2Polygon { hit(3424); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3425(_a0: usize, _a1: usize, _a2: usize) -> S_b2Polygon { hit(3425); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3426(_a0: i32, _a1: i32) -> i64 { hit(3426); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3427() -> i64 { hit(3427); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3428(_a0: u32) -> i64 { hit(3428); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3429(_a0: u32) -> i64 { hit(3429); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3430(_a0: usize, _a1: i64) -> i64 { hit(3430); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3431(_a0: usize, _a1: i64, _a2: i32, _a3: i32) -> i64 { hit(3431); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3432(_a0: u32, _a1: i64, _a2: i32, _a3: i32) -> i64 { hit(3432); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3433(_a0: u32, _a1: usize, _a2: i32, _a3: i64, _a4: usize) -> i64 { hit(3433); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3434(_a0: u64) { hit(3434);  }
unsafe extern "C" fn s3435() -> i32 { hit(3435); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3436(_a0: usize) -> i32 { hit(3436); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3437() -> usize { hit(3437); crate::trace::object(3437) }
unsafe extern "C" fn s3438(_a0: usize) { hit(3438);  }
unsafe extern "C" fn s3439() -> usize { hit(3439); crate::trace::object(3439) }
unsafe extern "C" fn s3440(_a0: usize) -> usize { hit(3440); crate::trace::object(3440) }
unsafe extern "C" fn s3441() -> u32 { hit(3441); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3442(_a0: usize) -> u32 { hit(3442); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3443(_a0: u32) -> f32 { hit(3443); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3444(_a0: u32, _a1: usize) -> i32 { hit(3444); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3445(_a0: u32, _a1: usize) -> i32 { hit(3445); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3446(_a0: u32) -> usize { hit(3446); crate::trace::object(3446) }
unsafe extern "C" fn s3447(_a0: u32) -> usize { hit(3447); crate::trace::object(3447) }
unsafe extern "C" fn s3448(_a0: u32, _a1: usize) -> usize { hit(3448); crate::trace::object(3448) }
unsafe extern "C" fn s3449(_a0: u32, _a1: i32, _a2: i32, _a3: f32, _a4: i32, _a5: usize) -> i32 { hit(3449); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3450() -> usize { hit(3450); crate::trace::object(3450) }
unsafe extern "C" fn s3451() -> u32 { hit(3451); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3452(_a0: u32) { hit(3452);  }
unsafe extern "C" fn s3453(_a0: usize) -> u32 { hit(3453); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3454(_a0: u32, _a1: usize, _a2: usize) -> usize { hit(3454); crate::trace::object(3454) }
unsafe extern "C" fn s3455(_a0: u32, _a1: usize, _a2: usize) -> i32 { hit(3455); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3456(_a0: u32, _a1: usize, _a2: i64) -> i32 { hit(3456); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3457(_a0: u32, _a1: usize, _a2: i32) -> i32 { hit(3457); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3458(_a0: u32, _a1: usize, _a2: i32) -> i32 { hit(3458); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3459(_a0: u32) -> usize { hit(3459); crate::trace::object(3459) }
unsafe extern "C" fn s3460(_a0: usize) -> u32 { hit(3460); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3461(_a0: usize, _a1: usize) -> i32 { hit(3461); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3462(_a0: usize, _a1: i32, _a2: i32, _a3: i64) -> usize { hit(3462); crate::trace::object(3462) }
unsafe extern "C" fn s3463(_a0: usize, _a1: i32, _a2: i32, _a3: i64) -> usize { hit(3463); crate::trace::object(3463) }
unsafe extern "C" fn s3464(_a0: usize) { hit(3464);  }
unsafe extern "C" fn s3465(_a0: usize, _a1: usize) -> i32 { hit(3465); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3466(_a0: usize, _a1: usize, _a2: usize) -> i32 { hit(3466); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3467(_a0: usize, _a1: usize) -> i32 { hit(3467); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3468(_a0: usize) -> i32 { hit(3468); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3469(_a0: usize) -> i32 { hit(3469); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3470(_a0: usize) -> i32 { hit(3470); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3471(_a0: usize) -> i32 { hit(3471); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3472(_a0: usize) -> i32 { hit(3472); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3473(_a0: usize) -> i32 { hit(3473); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3474(_a0: usize, _a1: usize, _a2: usize) -> i32 { hit(3474); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3475(_a0: usize) -> f32 { hit(3475); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3476(_a0: usize, _a1: i32, _a2: i32) -> i32 { hit(3476); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3477(_a0: usize, _a1: i32, _a2: i32) -> i32 { hit(3477); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3478(_a0: usize, _a1: i32) -> i32 { hit(3478); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3479(_a0: usize, _a1: f32) -> i32 { hit(3479); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3480(_a0: usize, _a1: f32, _a2: f32) -> i32 { hit(3480); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3481(_a0: usize, _a1: usize) -> i32 { hit(3481); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3482(_a0: usize, _a1: i32) -> i32 { hit(3482); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3483(_a0: usize, _a1: i64) -> i32 { hit(3483); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3484(_a0: usize, _a1: i32, _a2: i32) -> i32 { hit(3484); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3485(_a0: usize, _a1: usize, _a2: usize) -> i32 { hit(3485); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3486(_a0: usize) -> i64 { hit(3486); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3487(_a0: usize) -> f32 { hit(3487); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3488(_a0: usize, _a1: i32, _a2: i32) -> i32 { hit(3488); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3489(_a0: usize, _a1: i32) -> i32 { hit(3489); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3490(_a0: usize, _a1: i32) -> i32 { hit(3490); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3491(_a0: usize, _a1: i32) -> i32 { hit(3491); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3492(_a0: usize) -> i32 { hit(3492); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3493(_a0: usize, _a1: i32, _a2: i32, _a3: i32, _a4: i32, _a5: i64) -> usize { hit(3493); crate::trace::object(3493) }
unsafe extern "C" fn s3494(_a0: i64) -> i32 { hit(3494); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3495(_a0: i64) { hit(3495);  }
unsafe extern "C" fn s3496(_a0: usize) -> i32 { hit(3496); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3497(_a0: i32) { hit(3497);  }
unsafe extern "C" fn s3498(_a0: i32) { hit(3498);  }
unsafe extern "C" fn s3499() -> i32 { hit(3499); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3500() -> i32 { hit(3500); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3501(_a0: i64) -> usize { hit(3501); crate::trace::object(3501) }
unsafe extern "C" fn s3502(_a0: usize, _a1: i32, _a2: i32) -> usize { hit(3502); crate::trace::object(3502) }
unsafe extern "C" fn s3503(_a0: usize) { hit(3503);  }
unsafe extern "C" fn s3504(_a0: usize) -> i32 { hit(3504); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3505() -> i32 { hit(3505); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3506() -> i32 { hit(3506); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3507(_a0: u32) -> usize { hit(3507); crate::trace::object(3507) }
unsafe extern "C" fn s3508(_a0: u32) -> usize { hit(3508); crate::trace::object(3508) }
unsafe extern "C" fn s3509(_a0: usize) { hit(3509);  }
unsafe extern "C" fn s3510(_a0: usize, _a1: i64, _a2: i32) -> i32 { hit(3510); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3511(_a0: usize) -> usize { hit(3511); crate::trace::object(3511) }
unsafe extern "C" fn s3512(_a0: usize) -> i64 { hit(3512); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3513(_a0: usize, _a1: u8, _a2: u8, _a3: u8) -> i32 { hit(3513); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3514(_a0: usize, _a1: u16, _a2: u16, _a3: u32) -> i32 { hit(3514); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3515(_a0: usize, _a1: u16, _a2: u16, _a3: u32) -> i32 { hit(3515); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3516(_a0: usize, _a1: i64) -> i32 { hit(3516); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3517(_a0: usize, _a1: i64, _a2: usize, _a3: i32) -> i32 { hit(3517); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3518(_a0: usize) -> i32 { hit(3518); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3519(_a0: usize, _a1: usize) -> i32 { hit(3519); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3520(_a0: usize, _a1: usize) { hit(3520);  }
unsafe extern "C" fn s3521(_a0: usize) -> usize { hit(3521); crate::trace::object(3521) }
unsafe extern "C" fn s3522(_a0: usize, _a1: usize, _a2: usize) -> i32 { hit(3522); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3523(_a0: usize, _a1: usize) -> u32 { hit(3523); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3524(_a0: usize, _a1: usize) -> u32 { hit(3524); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3525(_a0: i32) -> i32 { hit(3525); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3526() -> usize { hit(3526); crate::trace::object(3526) }
unsafe extern "C" fn s3527() -> usize { hit(3527); crate::trace::object(3527) }
unsafe extern "C" fn s3528(_a0: usize, _a1: i32) -> i32 { hit(3528); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3529(_a0: usize, _a1: f32, _a2: f32) { hit(3529);  }
unsafe extern "C" fn s3530() -> u32 { hit(3530); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3531(_a0: i32, _a1: u32, _a2: i32) -> u32 { hit(3531); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3532(_a0: u32) -> usize { hit(3532); crate::trace::object(3532) }
unsafe extern "C" fn s3533(_a0: u32) { hit(3533);  }
unsafe extern "C" fn s3534(_a0: usize) -> i32 { hit(3534); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3535(_a0: usize) -> i32 { hit(3535); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3536(_a0: usize) -> i32 { hit(3536); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3537(_a0: usize, _a1: usize, _a2: i32) -> i32 { hit(3537); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3538(_a0: usize) -> i32 { hit(3538); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3539(_a0: i32, _a1: i32, _a2: u32, _a3: usize, _a4: i32) -> usize { hit(3539); crate::trace::object(3539) }
unsafe extern "C" fn s3540(_a0: usize) { hit(3540);  }
unsafe extern "C" fn s3541(_a0: usize, _a1: usize) -> i32 { hit(3541); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3542(_a0: usize) -> i32 { hit(3542); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3543(_a0: usize) -> i32 { hit(3543); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3544(_a0: usize, _a1: u8, _a2: u8, _a3: u8, _a4: u8) -> i32 { hit(3544); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3545(_a0: usize, _a1: i32, _a2: i32, _a3: usize, _a4: i32) -> i32 { hit(3545); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3546(_a0: usize, _a1: usize, _a2: usize, _a3: usize) { hit(3546);  }
unsafe extern "C" fn s3547(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: i32, _a5: usize, _a6: i32) { hit(3547);  }
unsafe extern "C" fn s3548(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: i32, _a5: usize) { hit(3548);  }
unsafe extern "C" fn s3549(_a0: usize, _a1: usize) -> i32 { hit(3549); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3550(_a0: usize, _a1: usize, _a2: usize) -> i32 { hit(3550); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3551(_a0: usize, _a1: i32) { hit(3551);  }
unsafe extern "C" fn s3552(_a0: usize, _a1: i32) { hit(3552);  }
unsafe extern "C" fn s3553(_a0: usize, _a1: i32) { hit(3553);  }
unsafe extern "C" fn s3554(_a0: usize, _a1: i32) { hit(3554);  }
unsafe extern "C" fn s3555(_a0: usize) { hit(3555);  }
unsafe extern "C" fn s3556(_a0: usize) { hit(3556);  }
unsafe extern "C" fn s3557(_a0: usize, _a1: i32) { hit(3557);  }
unsafe extern "C" fn s3558(_a0: usize, _a1: i32) { hit(3558);  }
unsafe extern "C" fn s3559(_a0: usize, _a1: i32, _a2: i32, _a3: usize, _a4: usize) { hit(3559);  }
unsafe extern "C" fn s3560() -> usize { hit(3560); crate::trace::object(3560) }
unsafe extern "C" fn s3561() { hit(3561); unsafe { crate::impls::globalSteam_SteamAPI_RunCallbacks() } }
unsafe extern "C" fn s3562() { hit(3562); unsafe { crate::impls::globalSteam_SteamGameServer_RunCallbacks() } }
unsafe extern "C" fn s3563() -> usize { hit(3563); crate::trace::object(3563) }
unsafe extern "C" fn s3564() -> usize { hit(3564); crate::trace::object(3564) }
unsafe extern "C" fn s3565() -> usize { hit(3565); crate::trace::object(3565) }
unsafe extern "C" fn s3566() -> usize { hit(3566); crate::trace::object(3566) }
unsafe extern "C" fn s3567() -> usize { hit(3567); crate::trace::object(3567) }
unsafe extern "C" fn s3568() -> usize { hit(3568); crate::trace::object(3568) }
unsafe extern "C" fn s3569() -> usize { hit(3569); crate::trace::object(3569) }
unsafe extern "C" fn s3570() -> usize { hit(3570); crate::trace::object(3570) }
unsafe extern "C" fn s3571() -> usize { hit(3571); crate::trace::object(3571) }
unsafe extern "C" fn s3572() -> i32 { hit(3572); unsafe { core::mem::zeroed() } }
unsafe extern "C" fn s3573() -> u64 { hit(3573); unsafe { crate::impls::globalSteam_SteamGameServer_GetSteamID() } }
unsafe extern "C" fn s3574() { hit(3574);  }
unsafe extern "C" fn s3575() { hit(3575);  }
unsafe extern "C" fn s3576() -> usize { hit(3576); crate::trace::object(3576) }
unsafe extern "C" fn s3577(_a0: i32, _a1: i32, _a2: usize) { hit(3577);  }

pub static NAMES: [&str; 3578] = [
    "Debug_Error",
    "AggregateGlue_CreateAggregateSceneObject",
    "AggregateGlue_GetMaxFragmentCount",
    "CnmtnGrpBldr_DeleteThis",
    "CnmtnGrpBldr_Create",
    "CnmtnGrpBldr_AddAnimation",
    "CnmtnGrpBldr_AddFrame",
    "CnmtnGrpBldr_SetName",
    "CnmtnGrpBldr_SetFrameRate",
    "CnmtnGrpBldr_SetLooping",
    "CnmtnGrpBldr_SetDelta",
    "CnmtnGrpBldr_SetDisableInterpolation",
    "CAttachment_GetInfluenceName",
    "CAttachment_GetInfluenceOffset",
    "CAttachment_GetInfluenceRotation",
    "Get__CAttachment_m_name",
    "Set__CAttachment_m_name",
    "Get__CAttachment_m_nInfluences",
    "Set__CAttachment_m_nInfluences",
    "Get__CAttachment_m_bIgnoreRotation",
    "Set__CAttachment_m_bIgnoreRotation",
    "CAudioMixBuffer_Create",
    "CAudioMixBuffer_Dispose",
    "CAudioMixBuffer_GetDataPointer",
    "CAudioMixBuffer_Silence",
    "CAudioMixBuffer_AbsLevel",
    "CAudioMixBuffer_AvergeLevel",
    "CAudioMixBuffer_Ramp",
    "CAudioMixBuffer_CopyFrom",
    "CAudioMixBuffer_Mix",
    "CAudioMixBuffer_MixRamp",
    "CdMxDvcBffrs_Create",
    "CdMxDvcBffrs_Destroy",
    "CdMxDvcBffrs_GetBuffer",
    "CAudioMixer_Dispose",
    "CAudioMixer_GetSamplePosition",
    "CAudioMixer_ShouldContinueMixing",
    "CAudioMixer_SetSamplePosition",
    "CAudioMixer_SetSampleEnd",
    "CAudioMixer_DelayOrSkipSamples",
    "CAudioMixer_IsReadyToMix",
    "CAudioMixer_GetPositionForSave",
    "CAudioMixer_SetPositionFromSaved",
    "CAudioMixer_UpdateMixerState",
    "CAudioMixer_GetIndexState",
    "CAudioMixer_GetSfxTable",
    "CAudioMixer_GetSampleCount",
    "CAudioMixer_GetChannelCount",
    "CAudioMixer_SetTimeScale",
    "CAudioMixer_EnableLooping",
    "CAudioMixer_ReadToBuffer",
    "CAudioProcessor_Dispose",
    "CAudioProcessor_Process",
    "CAudioProcessor_SetControlParameter",
    "CAudioProcessor_SetNameParameter",
    "CAudioProcessor_CreateDelay",
    "CAudioProcessor_CreatePitchShift",
    "CdStrmMngd_Create",
    "CdStrmMngd_Destroy",
    "CdStrmMngd_WriteAudioData",
    "CdStrmMngd_QueuedSampleCount",
    "CdStrmMngd_MaxWriteSampleCount",
    "CdStrmMngd_LatencySamplesCount",
    "CdStrmMngd_Pause",
    "CdStrmMngd_Resume",
    "CdStrmMngd_Close",
    "CdStrmMngd_GetName",
    "CdStrmMngd_GetSfxTable",
    "CBinauralEffect_Create",
    "CBinauralEffect_Dispose",
    "CBinauralEffect_Apply",
    "CBuilderHitBox_SetBoneName",
    "CBuilderHitBox_SetSurface",
    "CBuilderHitBox_AddTag",
    "CBuilderHitBox_SetBox",
    "CBuilderHitBox_SetSphere",
    "CBuilderHitBox_SetCapsule",
    "CBldrMtrlGrp_SetName",
    "CBldrMtrlGrp_AddMaterial",
    "CCameraRenderer_DeleteThis",
    "CCameraRenderer_Create",
    "CCameraRenderer_ClearSceneWorlds",
    "CCameraRenderer_AddSceneWorld",
    "CCameraRenderer_SetRenderAttributes",
    "CCameraRenderer_Render",
    "CCameraRenderer_RenderToTexture",
    "CCameraRenderer_RenderToCubeTexture",
    "CCameraRenderer_RenderToBitmap",
    "CCameraRenderer_RenderOverlay",
    "CCameraRenderer_RenderStereo",
    "CCameraRenderer_SubmitStereo",
    "CCameraRenderer_BlitStereo",
    "CCameraRenderer_ClearRenderTags",
    "CCameraRenderer_ClearExcludeTags",
    "CCameraRenderer_AddRenderTag",
    "CCameraRenderer_AddExcludeTag",
    "Get__CCameraRenderer_RenderedProjectionToView",
    "Set__CCameraRenderer_RenderedProjectionToView",
    "Get__CCameraRenderer_RenderedViewport",
    "Set__CCameraRenderer_RenderedViewport",
    "Get__CCameraRenderer_RenderedDepthRange",
    "Set__CCameraRenderer_RenderedDepthRange",
    "Get__CCameraRenderer_ViewUniqueId",
    "Set__CCameraRenderer_ViewUniqueId",
    "Get__CCameraRenderer_CameraPosition",
    "Set__CCameraRenderer_CameraPosition",
    "Get__CCameraRenderer_CameraRotation",
    "Set__CCameraRenderer_CameraRotation",
    "Get__CCameraRenderer_FieldOfView",
    "Set__CCameraRenderer_FieldOfView",
    "Get__CCameraRenderer_ZNear",
    "Set__CCameraRenderer_ZNear",
    "Get__CCameraRenderer_ZFar",
    "Set__CCameraRenderer_ZFar",
    "Get__CCameraRenderer_Rect",
    "Set__CCameraRenderer_Rect",
    "Get__CCameraRenderer_Viewport",
    "Set__CCameraRenderer_Viewport",
    "Get__CCameraRenderer_ClipSpaceBounds",
    "Set__CCameraRenderer_ClipSpaceBounds",
    "Get__CCameraRenderer_EnablePostprocessing",
    "Set__CCameraRenderer_EnablePostprocessing",
    "Get__CCameraRenderer_EnableEngineOverlays",
    "Set__CCameraRenderer_EnableEngineOverlays",
    "Get__CCameraRenderer_EnableUI",
    "Set__CCameraRenderer_EnableUI",
    "Get__CCameraRenderer_UIOnly",
    "Set__CCameraRenderer_UIOnly",
    "Get__CCameraRenderer_Ortho",
    "Set__CCameraRenderer_Ortho",
    "Get__CCameraRenderer_OrthoSize",
    "Set__CCameraRenderer_OrthoSize",
    "Get__CCameraRenderer_NeedTonemapRenderer",
    "Set__CCameraRenderer_NeedTonemapRenderer",
    "Get__CCameraRenderer_SceneViewFlags",
    "Set__CCameraRenderer_SceneViewFlags",
    "Get__CCameraRenderer_IsRenderingStereo",
    "Set__CCameraRenderer_IsRenderingStereo",
    "Get__CCameraRenderer_MiddleEyePosition",
    "Set__CCameraRenderer_MiddleEyePosition",
    "Get__CCameraRenderer_MiddleEyeRotation",
    "Set__CCameraRenderer_MiddleEyeRotation",
    "Get__CCameraRenderer_OverrideProjection",
    "Set__CCameraRenderer_OverrideProjection",
    "Get__CCameraRenderer_HasOverrideProjection",
    "Set__CCameraRenderer_HasOverrideProjection",
    "Get__CCameraRenderer_FlipX",
    "Set__CCameraRenderer_FlipX",
    "Get__CCameraRenderer_FlipY",
    "Set__CCameraRenderer_FlipY",
    "From_CSceneObject_To_CDecalSceneObject",
    "To_CSceneObject_From_CDecalSceneObject",
    "CDclScnbjct_ChangeFlags",
    "CDclScnbjct_SetFlags",
    "CDclScnbjct_HasFlags",
    "CDclScnbjct_GetFlags",
    "CDclScnbjct_GetOriginalFlags",
    "CDclScnbjct_ClearFlags",
    "CDclScnbjct_SetCullDistance",
    "CDclScnbjct_EnableLightingCache",
    "CDclScnbjct_SetLightingOrigin",
    "CDclScnbjct_GetLightingOrigin",
    "CDclScnbjct_HasLightingOrigin",
    "CDclScnbjct_SetTintRGBA",
    "CDclScnbjct_GetTintRGBA",
    "CDclScnbjct_SetAlphaFade",
    "CDclScnbjct_GetAlphaFade",
    "CDclScnbjct_SetMaterialOverrideForMeshInstances",
    "CDclScnbjct_ClearMaterialOverrideList",
    "CDclScnbjct_SetMaterialOverride",
    "CDclScnbjct_IsLoaded",
    "CDclScnbjct_IsRenderingEnabled",
    "CDclScnbjct_SetLoaded",
    "CDclScnbjct_ClearLoaded",
    "CDclScnbjct_DisableRendering",
    "CDclScnbjct_EnableRendering",
    "CDclScnbjct_SetRenderingEnabled",
    "CDclScnbjct_GetBoundingSphereRadius",
    "CDclScnbjct_SetTransform",
    "CDclScnbjct_GetCTransform",
    "CDclScnbjct_SetBounds",
    "CDclScnbjct_GetBounds",
    "CDclScnbjct_SetBoundsInfinite",
    "CDclScnbjct_GetParent",
    "CDclScnbjct_AddChildObject",
    "CDclScnbjct_RemoveChild",
    "CDclScnbjct_GetAttributesPtrForModify",
    "CDclScnbjct_EnableMeshGroups",
    "CDclScnbjct_DisableMeshGroups",
    "CDclScnbjct_ResetMeshGroups",
    "CDclScnbjct_GetCurrentMeshGroupMask",
    "CDclScnbjct_GetWorld",
    "CDclScnbjct_SetLOD",
    "CDclScnbjct_DisableLOD",
    "CDclScnbjct_GetCurrentLODGroupMask",
    "CDclScnbjct_GetCurrentLODLevel",
    "CDclScnbjct_GetModelHandle",
    "CDclScnbjct_SetMaterialGroup",
    "CDclScnbjct_SetBodyGroup",
    "CDclScnbjct_SetBatchable",
    "CDclScnbjct_IsNotBatchable",
    "CDclScnbjct_SetUniqueBatchGroup",
    "CDclScnbjct_RemoveTag",
    "CDclScnbjct_RemoveAllTags",
    "CDclScnbjct_GetTagCount",
    "CDclScnbjct_GetTagAt",
    "CDclScnbjct_AddTag",
    "CDclScnbjct_HasTag",
    "CDclScnbjct_SetForceLayerID",
    "CDclScnbjct_SetLayerMatchID",
    "CDclScnbjct_GetLayerMatchIDValue",
    "CDclScnbjct_GetIntValue",
    "CDclScnbjct_UpdateFlagsBasedOnMaterial",
    "CDclScnbjct_SetMaterialOverrideByIndex",
    "Get__CDclScnbjct_m_hColor",
    "Set__CDclScnbjct_m_hColor",
    "Get__CDclScnbjct_m_hNormal",
    "Set__CDclScnbjct_m_hNormal",
    "Get__CDclScnbjct_m_hRMO",
    "Set__CDclScnbjct_m_hRMO",
    "Get__CDclScnbjct_m_nSortOrder",
    "Set__CDclScnbjct_m_nSortOrder",
    "Get__CDclScnbjct_m_nExclusionBitMask",
    "Set__CDclScnbjct_m_nExclusionBitMask",
    "Get__CDclScnbjct_m_vColorTint",
    "Set__CDclScnbjct_m_vColorTint",
    "Get__CDclScnbjct_m_flAttenuationAngle",
    "Set__CDclScnbjct_m_flAttenuationAngle",
    "Get__CDclScnbjct_m_flColorMix",
    "Set__CDclScnbjct_m_flColorMix",
    "Get__CDclScnbjct_m_hEmission",
    "Set__CDclScnbjct_m_hEmission",
    "Get__CDclScnbjct_m_flEmissionEnergy",
    "Set__CDclScnbjct_m_flEmissionEnergy",
    "Get__CDclScnbjct_m_nSequenceIndex",
    "Set__CDclScnbjct_m_nSequenceIndex",
    "Get__CDclScnbjct_m_hHeight",
    "Set__CDclScnbjct_m_hHeight",
    "Get__CDclScnbjct_m_flParallaxStrength",
    "Set__CDclScnbjct_m_flParallaxStrength",
    "Get__CDclScnbjct_m_nSamplerIndex",
    "Set__CDclScnbjct_m_nSamplerIndex",
    "Get__CDclScnbjct_m_flCoverageAmount",
    "Set__CDclScnbjct_m_flCoverageAmount",
    "Get__CDclScnbjct_m_flCoverageRange",
    "Set__CDclScnbjct_m_flCoverageRange",
    "CDirectEffect_Create",
    "CDirectEffect_Dispose",
    "CDirectEffect_Apply",
    "From_CSceneObject_To_CDynamicSceneObject",
    "To_CSceneObject_From_CDynamicSceneObject",
    "CDynmcScnbjct_Create",
    "CDynmcScnbjct_Begin",
    "CDynmcScnbjct_End",
    "CDynmcScnbjct_AddVertex",
    "CDynmcScnbjct_AddVertexRange",
    "CDynmcScnbjct_Reset",
    "CDynmcScnbjct_ChangeFlags",
    "CDynmcScnbjct_SetFlags",
    "CDynmcScnbjct_HasFlags",
    "CDynmcScnbjct_GetFlags",
    "CDynmcScnbjct_GetOriginalFlags",
    "CDynmcScnbjct_ClearFlags",
    "CDynmcScnbjct_SetCullDistance",
    "CDynmcScnbjct_EnableLightingCache",
    "CDynmcScnbjct_SetLightingOrigin",
    "CDynmcScnbjct_GetLightingOrigin",
    "CDynmcScnbjct_HasLightingOrigin",
    "CDynmcScnbjct_SetTintRGBA",
    "CDynmcScnbjct_GetTintRGBA",
    "CDynmcScnbjct_SetAlphaFade",
    "CDynmcScnbjct_GetAlphaFade",
    "CDynmcScnbjct_SetMaterialOverrideForMeshInstances",
    "CDynmcScnbjct_ClearMaterialOverrideList",
    "CDynmcScnbjct_SetMaterialOverride",
    "CDynmcScnbjct_IsLoaded",
    "CDynmcScnbjct_IsRenderingEnabled",
    "CDynmcScnbjct_SetLoaded",
    "CDynmcScnbjct_ClearLoaded",
    "CDynmcScnbjct_DisableRendering",
    "CDynmcScnbjct_EnableRendering",
    "CDynmcScnbjct_SetRenderingEnabled",
    "CDynmcScnbjct_GetBoundingSphereRadius",
    "CDynmcScnbjct_SetTransform",
    "CDynmcScnbjct_GetCTransform",
    "CDynmcScnbjct_SetBounds",
    "CDynmcScnbjct_GetBounds",
    "CDynmcScnbjct_SetBoundsInfinite",
    "CDynmcScnbjct_GetParent",
    "CDynmcScnbjct_AddChildObject",
    "CDynmcScnbjct_RemoveChild",
    "CDynmcScnbjct_GetAttributesPtrForModify",
    "CDynmcScnbjct_EnableMeshGroups",
    "CDynmcScnbjct_DisableMeshGroups",
    "CDynmcScnbjct_ResetMeshGroups",
    "CDynmcScnbjct_GetCurrentMeshGroupMask",
    "CDynmcScnbjct_GetWorld",
    "CDynmcScnbjct_SetLOD",
    "CDynmcScnbjct_DisableLOD",
    "CDynmcScnbjct_GetCurrentLODGroupMask",
    "CDynmcScnbjct_GetCurrentLODLevel",
    "CDynmcScnbjct_GetModelHandle",
    "CDynmcScnbjct_SetMaterialGroup",
    "CDynmcScnbjct_SetBodyGroup",
    "CDynmcScnbjct_SetBatchable",
    "CDynmcScnbjct_IsNotBatchable",
    "CDynmcScnbjct_SetUniqueBatchGroup",
    "CDynmcScnbjct_RemoveTag",
    "CDynmcScnbjct_RemoveAllTags",
    "CDynmcScnbjct_GetTagCount",
    "CDynmcScnbjct_GetTagAt",
    "CDynmcScnbjct_AddTag",
    "CDynmcScnbjct_HasTag",
    "CDynmcScnbjct_SetForceLayerID",
    "CDynmcScnbjct_SetLayerMatchID",
    "CDynmcScnbjct_GetLayerMatchIDValue",
    "CDynmcScnbjct_GetIntValue",
    "CDynmcScnbjct_UpdateFlagsBasedOnMaterial",
    "CDynmcScnbjct_SetMaterialOverrideByIndex",
    "Get__CDynmcScnbjct_Material",
    "Set__CDynmcScnbjct_Material",
    "CEntityKeyValues_GetValueString",
    "CEntityKeyValues_GetKeyCount",
    "CEntityKeyValues_GetKey",
    "CEntityKeyValues_GetValueString_1",
    "From_CSceneLightObject_To_CEnvMapSceneObject",
    "To_CSceneLightObject_From_CEnvMapSceneObject",
    "From_CSceneObject_To_CEnvMapSceneObject",
    "To_CSceneObject_From_CEnvMapSceneObject",
    "CnvMpScnbjct_CalculateRadianceSH",
    "CnvMpScnbjct_CalculateRadianceSH_1",
    "CnvMpScnbjct_CalculateNormalizationSH",
    "CnvMpScnbjct_CalculateBounds",
    "CnvMpScnbjct_SetWorldPosition",
    "CnvMpScnbjct_GetWorldPosition",
    "CnvMpScnbjct_SetWorldDirection",
    "CnvMpScnbjct_GetWorldDirection",
    "CnvMpScnbjct_SetColor",
    "CnvMpScnbjct_SetBounceColor",
    "CnvMpScnbjct_GetColor",
    "CnvMpScnbjct_SetRadius",
    "CnvMpScnbjct_GetRadius",
    "CnvMpScnbjct_SetTheta",
    "CnvMpScnbjct_GetTheta",
    "CnvMpScnbjct_SetPhi",
    "CnvMpScnbjct_GetPhi",
    "CnvMpScnbjct_SetFallOff",
    "CnvMpScnbjct_GetFallOff",
    "CnvMpScnbjct_GetShadowTextureResolution",
    "CnvMpScnbjct_SetShadowTextureResolution",
    "CnvMpScnbjct_GetShadows",
    "CnvMpScnbjct_SetShadows",
    "CnvMpScnbjct_SetConstantAttn",
    "CnvMpScnbjct_GetConstantAttn",
    "CnvMpScnbjct_SetLinearAttn",
    "CnvMpScnbjct_GetLinearAttn",
    "CnvMpScnbjct_SetQuadraticAttn",
    "CnvMpScnbjct_GetQuadraticAttn",
    "CnvMpScnbjct_SetLightCookie",
    "CnvMpScnbjct_GetLightCookie",
    "CnvMpScnbjct_SetLightModulatorRange",
    "CnvMpScnbjct_GetShadowCascades",
    "CnvMpScnbjct_SetShadowCascades",
    "CnvMpScnbjct_GetShadowCascadeSplitRatio",
    "CnvMpScnbjct_SetShadowCascadeSplitRatio",
    "CnvMpScnbjct_GetCascadeDistanceScale",
    "CnvMpScnbjct_SetCascadeDistanceScale",
    "CnvMpScnbjct_GetFogContributionStength",
    "CnvMpScnbjct_SetFogContributionStength",
    "CnvMpScnbjct_GetFogLightingMode",
    "CnvMpScnbjct_SetFogLightingMode",
    "CnvMpScnbjct_SetBakeLightIndex",
    "CnvMpScnbjct_SetBakeLightIndexScale",
    "CnvMpScnbjct_SetUsesIndexedBakedLighting",
    "CnvMpScnbjct_SetRenderDiffuse",
    "CnvMpScnbjct_SetRenderSpecular",
    "CnvMpScnbjct_SetRenderTransmissive",
    "CnvMpScnbjct_SetLightSourceSize0",
    "CnvMpScnbjct_SetLightSourceSize1",
    "CnvMpScnbjct_SetShadowTextureWidth",
    "CnvMpScnbjct_SetShadowTextureHeight",
    "CnvMpScnbjct_GetShadowTextureWidth",
    "CnvMpScnbjct_GetShadowTextureHeight",
    "CnvMpScnbjct_GetLightFlags",
    "CnvMpScnbjct_SetLightFlags",
    "CnvMpScnbjct_GetLightShape",
    "CnvMpScnbjct_SetLightShape",
    "CnvMpScnbjct_SetLightSourceDim0",
    "CnvMpScnbjct_SetLightSourceDim1",
    "CnvMpScnbjct_GetLightType",
    "Get__CnvMpScnbjct_m_nProjectionMode",
    "Set__CnvMpScnbjct_m_nProjectionMode",
    "Get__CnvMpScnbjct_m_vBoxProjectMins",
    "Set__CnvMpScnbjct_m_vBoxProjectMins",
    "Get__CnvMpScnbjct_m_vBoxProjectMaxs",
    "Set__CnvMpScnbjct_m_vBoxProjectMaxs",
    "Get__CnvMpScnbjct_m_vColor",
    "Set__CnvMpScnbjct_m_vColor",
    "Get__CnvMpScnbjct_m_nRenderPriority",
    "Set__CnvMpScnbjct_m_nRenderPriority",
    "Get__CnvMpScnbjct_m_hEnvMapTexture",
    "Set__CnvMpScnbjct_m_hEnvMapTexture",
    "Get__CnvMpScnbjct_m_vNormalizationSH",
    "Set__CnvMpScnbjct_m_vNormalizationSH",
    "Get__CnvMpScnbjct_m_flFeathering",
    "Set__CnvMpScnbjct_m_flFeathering",
    "CFrustum_Create",
    "CFrustum_Delete",
    "CFrustum_InitCamera",
    "CFrustum_InitCamera_1",
    "CFrustum_InitOrthoCamera",
    "CFrustum_SetCameraWidthHeight",
    "CFrustum_BuildFrustumFromVectors",
    "CFrustum_GetView",
    "CFrustum_GetProj",
    "CFrustum_GetInvProj",
    "CFrustum_GetReverseZProj",
    "CFrustum_GetInvReverseZProj",
    "CFrustum_GetViewProj",
    "CFrustum_GetInvViewProj",
    "CFrustum_GetInvReverseZViewProj",
    "CFrustum_GetInvReverseZViewProjTranspose",
    "CFrustum_GetReverseZViewProj",
    "CFrustum_GetReverseZViewProjTranspose",
    "CFrustum_BoundingVolumeIntersectsFrustum",
    "CFrustum_ScreenTransform",
    "CFrustum_WorldToView",
    "CFrustum_ViewToWorld",
    "CFrustum_IsOrthographic",
    "CFrustum_GetCameraPosition",
    "CFrustum_GetCameraAngles",
    "CFrustum_GetCameraNearPlane",
    "CFrustum_GetCameraFarPlane",
    "CFrustum_GetCameraFOV",
    "CFrustum_GetCameraAspect",
    "CFrustum_GetPlane",
    "CFrustum_ComputeScreenSize",
    "CHitBox_GetTag",
    "Get__CHitBox_m_vMinBounds",
    "Set__CHitBox_m_vMinBounds",
    "Get__CHitBox_m_vMaxBounds",
    "Set__CHitBox_m_vMaxBounds",
    "Get__CHitBox_m_name",
    "Set__CHitBox_m_name",
    "Get__CHitBox_m_sSurfaceProperty",
    "Set__CHitBox_m_sSurfaceProperty",
    "Get__CHitBox_m_sBoneName",
    "Set__CHitBox_m_sBoneName",
    "Get__CHitBox_m_flShapeRadius",
    "Set__CHitBox_m_flShapeRadius",
    "Get__CHitBox_m_nBoneNameHash",
    "Set__CHitBox_m_nBoneNameHash",
    "Get__CHitBox_m_cRenderColor",
    "Set__CHitBox_m_cRenderColor",
    "Get__CHitBox_m_nHitBoxIndex",
    "Set__CHitBox_m_nHitBoxIndex",
    "Get__CHitBox_m_nShapeType",
    "Set__CHitBox_m_nShapeType",
    "Get__CHitBox_m_bForcedTransform",
    "Set__CHitBox_m_bForcedTransform",
    "Get__CHitBox_m_bTranslationOnly",
    "Set__CHitBox_m_bTranslationOnly",
    "Get__CHitBox_m_bVisible",
    "Set__CHitBox_m_bVisible",
    "Get__CHitBox_m_bSelected",
    "Set__CHitBox_m_bSelected",
    "CHitBoxSet_numhitboxes",
    "CHitBoxSet_pHitbox",
    "Get__CHitBoxSet_m_name",
    "Set__CHitBoxSet_m_name",
    "Get__CHitBoxSet_m_SourceFilename",
    "Set__CHitBoxSet_m_SourceFilename",
    "From_CSceneObject_To_CManagedSceneObject",
    "To_CSceneObject_From_CManagedSceneObject",
    "CMngdScnbjct_Create",
    "CMngdScnbjct_ChangeFlags",
    "CMngdScnbjct_SetFlags",
    "CMngdScnbjct_HasFlags",
    "CMngdScnbjct_GetFlags",
    "CMngdScnbjct_GetOriginalFlags",
    "CMngdScnbjct_ClearFlags",
    "CMngdScnbjct_SetCullDistance",
    "CMngdScnbjct_EnableLightingCache",
    "CMngdScnbjct_SetLightingOrigin",
    "CMngdScnbjct_GetLightingOrigin",
    "CMngdScnbjct_HasLightingOrigin",
    "CMngdScnbjct_SetTintRGBA",
    "CMngdScnbjct_GetTintRGBA",
    "CMngdScnbjct_SetAlphaFade",
    "CMngdScnbjct_GetAlphaFade",
    "CMngdScnbjct_SetMaterialOverrideForMeshInstances",
    "CMngdScnbjct_ClearMaterialOverrideList",
    "CMngdScnbjct_SetMaterialOverride",
    "CMngdScnbjct_IsLoaded",
    "CMngdScnbjct_IsRenderingEnabled",
    "CMngdScnbjct_SetLoaded",
    "CMngdScnbjct_ClearLoaded",
    "CMngdScnbjct_DisableRendering",
    "CMngdScnbjct_EnableRendering",
    "CMngdScnbjct_SetRenderingEnabled",
    "CMngdScnbjct_GetBoundingSphereRadius",
    "CMngdScnbjct_SetTransform",
    "CMngdScnbjct_GetCTransform",
    "CMngdScnbjct_SetBounds",
    "CMngdScnbjct_GetBounds",
    "CMngdScnbjct_SetBoundsInfinite",
    "CMngdScnbjct_GetParent",
    "CMngdScnbjct_AddChildObject",
    "CMngdScnbjct_RemoveChild",
    "CMngdScnbjct_GetAttributesPtrForModify",
    "CMngdScnbjct_EnableMeshGroups",
    "CMngdScnbjct_DisableMeshGroups",
    "CMngdScnbjct_ResetMeshGroups",
    "CMngdScnbjct_GetCurrentMeshGroupMask",
    "CMngdScnbjct_GetWorld",
    "CMngdScnbjct_SetLOD",
    "CMngdScnbjct_DisableLOD",
    "CMngdScnbjct_GetCurrentLODGroupMask",
    "CMngdScnbjct_GetCurrentLODLevel",
    "CMngdScnbjct_GetModelHandle",
    "CMngdScnbjct_SetMaterialGroup",
    "CMngdScnbjct_SetBodyGroup",
    "CMngdScnbjct_SetBatchable",
    "CMngdScnbjct_IsNotBatchable",
    "CMngdScnbjct_SetUniqueBatchGroup",
    "CMngdScnbjct_RemoveTag",
    "CMngdScnbjct_RemoveAllTags",
    "CMngdScnbjct_GetTagCount",
    "CMngdScnbjct_GetTagAt",
    "CMngdScnbjct_AddTag",
    "CMngdScnbjct_HasTag",
    "CMngdScnbjct_SetForceLayerID",
    "CMngdScnbjct_SetLayerMatchID",
    "CMngdScnbjct_GetLayerMatchIDValue",
    "CMngdScnbjct_GetIntValue",
    "CMngdScnbjct_UpdateFlagsBasedOnMaterial",
    "CMngdScnbjct_SetMaterialOverrideByIndex",
    "CMtrlSystm2ppSys_Create",
    "CMtrlSystm2ppSys_Destroy",
    "CMtrlSystm2ppSys_Init",
    "CMtrlSystm2ppSys_InitWithoutMaterialSystem",
    "CMtrlSystm2ppSys_InitFinishSetupMaterialSystem",
    "CMtrlSystm2ppSys_StartBackgroundSystems",
    "CMtrlSystm2ppSys_SuppressStartupManifestLoad",
    "CMtrlSystm2ppSys_SetModuleSearchPath",
    "CMtrlSystm2ppSys_SetModGameSubdir",
    "CMtrlSystm2ppSys_SetModFromFileName",
    "CMtrlSystm2ppSys_DisableModPathCheck",
    "CMtrlSystm2ppSys_SetDefaultRenderSystemOption",
    "CMtrlSystm2ppSys_SetInitializationPhase",
    "CMtrlSystm2ppSys_GetInitializationPhase",
    "CMtrlSystm2ppSys_PreShutdown",
    "CMtrlSystm2ppSys_InitSDL",
    "CMtrlSystm2ppSys_ShutdownSDL",
    "CMtrlSystm2ppSys_IsConsoleApp",
    "CMtrlSystm2ppSys_IsGameApp",
    "CMtrlSystm2ppSys_SetDedicatedServer",
    "CMtrlSystm2ppSys_IsDedicatedServer",
    "CMtrlSystm2ppSys_GetContentPath",
    "CMtrlSystm2ppSys_GetModGameSubdir",
    "CMtrlSystm2ppSys_SetInToolsMode",
    "CMtrlSystm2ppSys_IsInToolsMode",
    "CMtrlSystm2ppSys_IsInDeveloperMode",
    "CMtrlSystm2ppSys_IsInVRMode",
    "CMtrlSystm2ppSys_SuppressCOMInitialization",
    "CMtrlSystm2ppSys_IsRunningOnCustomerMachine",
    "CMtrlSystm2ppSys_AddSystem",
    "CMtrlSystm2ppSys_SetInTestMode",
    "CMtrlSystm2ppSys_IsInTestMode",
    "CMtrlSystm2ppSys_SetInStandaloneApp",
    "CMtrlSystm2ppSys_IsStandaloneApp",
    "CMtrlSystm2ppSys_SetSteamAppId",
    "CMtrlSystm2ppSys_GetSteamAppId",
    "CModel_DestroyStrongHandle",
    "CModel_IsStrongHandleValid",
    "CModel_IsError",
    "CModel_IsStrongHandleLoaded",
    "CModel_CopyStrongHandle",
    "CModel_GetBindingPtr",
    "CModel_GetGuid",
    "CModel_GetModelName",
    "CModel_GetBaseModelName",
    "CModel_GetBaseModel",
    "CModel_IsTranslucent",
    "CModel_IsTranslucentTwoPass",
    "CModel_HasPhysics",
    "CModel_FindModelSubKey",
    "CModel_GetAttachmentTransform",
    "CModel_GetAttachmentNameFromIndex",
    "CModel_GetBodyPartForName",
    "CModel_GetBodyPartName",
    "CModel_GetNumBodyParts",
    "CModel_GetNumBodyPartMeshes",
    "CModel_GetBodyPartMask",
    "CModel_GetBodyPartMeshMask",
    "CModel_FindMeshIndexForMask",
    "CModel_GetNumMeshGroups",
    "CModel_GetDefaultMeshGroupMask",
    "CModel_GetMeshGroupMaskForMesh",
    "CModel_GetBodyPartMeshName",
    "CModel_GetNumMaterialGroups",
    "CModel_GetMaterialGroupName",
    "CModel_GetMaterialGroupIndex",
    "CModel_GetNumMaterialsInGroup",
    "CModel_GetMaterialInGroup",
    "CModel_GetNumSceneObjects",
    "CModel_GetNumDrawCalls",
    "CModel_GetMaterial",
    "CModel_GetTotalMeshCounts",
    "CModel_GetMeshInfo",
    "CModel_GetMeshName",
    "CModel_GetDrawCallInfo",
    "CModel_GetDrawCallMaterial",
    "CModel_GetNumMeshes",
    "CModel_ComputeMaxLODLevelUsedByModel",
    "CModel_LODLevelForScreenSize",
    "CModel_GetLODSwitchDistanceCount",
    "CModel_GetLODSwitchDistance",
    "CModel_GetMeshBounds",
    "CModel_GetPhysicsBounds",
    "CModel_GetModelRenderBounds",
    "CModel_NumBones",
    "CModel_FindBoneIndex",
    "CModel_boneName",
    "CModel_boneParent",
    "CModel_GetBoneTransform",
    "CModel_bonePosParentSpace",
    "CModel_boneRotParentSpace",
    "CModel_NumFlexControllers",
    "CModel_GetFlexControllerName",
    "CModel_GetVisemeIndex",
    "CModel_GetVisemeMorph",
    "CModel_GetNumAnim",
    "CModel_GetAnimationName",
    "CModel_GetSequenceNames",
    "CModel_HasSceneObjects",
    "CModel_MeshTrace",
    "CModel_GetAnimationGraph",
    "CModel_GetPhysicsContainer",
    "CModel_FindHitboxSetByName",
    "CModel_FindHitboxSetIndexByName",
    "CModel_GetHitboxSetByIndex",
    "CModel_GetBoneIndexForHitbox",
    "CModel_GetHitboxSetCount",
    "CModel_HasSkinnedMeshes",
    "CModel_HasAnimationDrivenFlexes",
    "CModel_boneFlags",
    "CModel_GetNumAttachments",
    "CModel_GetAttachment",
    "CModel_GetMaterialIndexCount",
    "CModel_GetMaterialByIndex",
    "CModelBuilder_DeleteThis",
    "CModelBuilder_Create",
    "CModelBuilder_SetMass",
    "CModelBuilder_SetHullBounds",
    "CModelBuilder_SetViewBounds",
    "CModelBuilder_SetEyePosition",
    "CModelBuilder_SetMaxEyeDeflection",
    "CModelBuilder_SetModelKeyValues",
    "CModelBuilder_SetLODSwitchDistance",
    "CModelBuilder_AddRunTimeMesh",
    "CModelBuilder_AddRunTimeBone",
    "CModelBuilder_AddRunTimeAttachment",
    "CModelBuilder_AddMeshGroup",
    "CModelBuilder_AddMaterialGroup",
    "CModelBuilder_AddHitBox",
    "CModelBuilder_SetDefaultMeshGroupMask",
    "CModelBuilder_SetPhysics_Runtime",
    "CModelBuilder_AppendSkeletonsFromChildResources",
    "CModelBuilder_AddAnimationGroupRuntime",
    "CModelBuilder_SetTraceMesh",
    "CModelBuilder_CreateRuntimeModel",
    "CPathEffect_Create",
    "CPathEffect_Dispose",
    "CPathEffect_Apply",
    "CPhysBodyDesc_SetBoneName",
    "CPhysBodyDesc_SetSurface",
    "CPhysBodyDesc_SetBindPose",
    "CPhysBodyDesc_AddSphere",
    "CPhysBodyDesc_AddCapsule",
    "CPhysBodyDesc_AddBox",
    "CPhysBodyDesc_AddHull",
    "CPhysBodyDesc_AddHull_1",
    "CPhysBodyDesc_AddMesh",
    "Get__CPhysBodyDesc_m_flMass",
    "Set__CPhysBodyDesc_m_flMass",
    "Get__CPhysBodyDesc_m_flInertiaScale",
    "Set__CPhysBodyDesc_m_flInertiaScale",
    "Get__CPhysBodyDesc_m_flLinearDamping",
    "Set__CPhysBodyDesc_m_flLinearDamping",
    "Get__CPhysBodyDesc_m_flAngularDamping",
    "Set__CPhysBodyDesc_m_flAngularDamping",
    "CPhysBdyDscrry_DeleteThis",
    "CPhysBdyDscrry_Create",
    "CPhysBdyDscrry_Get",
    "CPhysBdyDscrry_GetJoint",
    "CPhysBdyDscrry_AddSurface",
    "CPhysSrfcPrprts_UpdatePhysics",
    "Get__CPhysSrfcPrprts_m_name",
    "Set__CPhysSrfcPrprts_m_name",
    "Get__CPhysSrfcPrprts_m_nameHash",
    "Set__CPhysSrfcPrprts_m_nameHash",
    "Get__CPhysSrfcPrprts_m_baseNameHash",
    "Set__CPhysSrfcPrprts_m_baseNameHash",
    "Get__CPhysSrfcPrprts_m_nIndex",
    "Set__CPhysSrfcPrprts_m_nIndex",
    "Get__CPhysSrfcPrprts_m_nBaseIndex",
    "Set__CPhysSrfcPrprts_m_nBaseIndex",
    "Get__CPhysSrfcPrprts_m_AudioSurface",
    "Set__CPhysSrfcPrprts_m_AudioSurface",
    "Get__CPhysSrfcPrprts_m_bHasScrapeSounds",
    "Set__CPhysSrfcPrprts_m_bHasScrapeSounds",
    "Get__CPhysSrfcPrprts_m_bHidden",
    "Set__CPhysSrfcPrprts_m_bHidden",
    "Get__CPhysSrfcPrprts_m_description",
    "Set__CPhysSrfcPrprts_m_description",
    "CPhysicsData_DestroyStrongHandle",
    "CPhysicsData_IsStrongHandleValid",
    "CPhysicsData_IsError",
    "CPhysicsData_IsStrongHandleLoaded",
    "CPhysicsData_CopyStrongHandle",
    "CPhysicsData_GetBindingPtr",
    "CPhysicsData_GetGuid",
    "CPhysicsData_GetResourceName",
    "CPhysicsData_GetChecksum",
    "CPhysicsData_GetBoneCount",
    "CPhysicsData_GetBoneName",
    "CPhysicsData_GetPartCount",
    "CPhysicsData_GetBoneHash",
    "CPhysicsData_GetBindPose",
    "CPhysicsData_GetPart",
    "CPhysicsData_GetSurfaceProperties",
    "CPhysicsData_GetSurfacePropertiesCount",
    "CPhysicsData_GetCollisionAttributeCount",
    "CPhysicsData_GetTag",
    "CPhysicsData_GetTagCount",
    "CPhysicsData_GetJointCount",
    "CPhysicsData_GetJoint",
    "Get__CPhysicsData_m_nFlags",
    "Set__CPhysicsData_m_nFlags",
    "CQueryResult_DeleteThis",
    "CQueryResult_Create",
    "CQueryResult_Count",
    "CQueryResult_Element",
    "CQueryResult_RemoveAll",
    "From_IReadBufferCallback_To_CReadBufferManagedCallback",
    "To_IReadBufferCallback_From_CReadBufferManagedCallback",
    "CRdBffrMngdCllbc_DeleteThis",
    "CRdBffrMngdCllbc_Create",
    "CRdBffrMngdCllbc_SetManagedId",
    "CRdBffrMngdCllbc_GetManagedId",
    "CRdBffrMngdCllbc_Done",
    "From_IReadTexturePixelsCallback_To_CReadTexturePixelsManagedCallback",
    "To_IReadTexturePixelsCallback_From_CReadTexturePixelsManagedCallback",
    "CRdTxtrPxlsMngdC_DeleteThis",
    "CRdTxtrPxlsMngdC_Create",
    "CRdTxtrPxlsMngdC_SetManagedId",
    "CRdTxtrPxlsMngdC_GetManagedId",
    "CRdTxtrPxlsMngdC_Done",
    "CRndrttrbts_DeleteThis",
    "CRndrttrbts_Create",
    "CRndrttrbts_SetFloatValue",
    "CRndrttrbts_GetFloatValue",
    "CRndrttrbts_DeleteFloatValue",
    "CRndrttrbts_SetVector2DValue",
    "CRndrttrbts_GetVector2DValue",
    "CRndrttrbts_DeleteVector2DValue",
    "CRndrttrbts_SetVectorValue",
    "CRndrttrbts_GetVectorValue",
    "CRndrttrbts_DeleteVectorValue",
    "CRndrttrbts_SetVector4DValue",
    "CRndrttrbts_GetVector4DValue",
    "CRndrttrbts_DeleteVector4DValue",
    "CRndrttrbts_SetVMatrixValue",
    "CRndrttrbts_GetVMatrixValue",
    "CRndrttrbts_DeleteVMatrixValue",
    "CRndrttrbts_SetStringValue",
    "CRndrttrbts_DeleteStringValue",
    "CRndrttrbts_SetIntValue",
    "CRndrttrbts_GetIntValue",
    "CRndrttrbts_DeleteIntValue",
    "CRndrttrbts_SetComboValue",
    "CRndrttrbts_GetComboValue",
    "CRndrttrbts_DeleteComboValue",
    "CRndrttrbts_SetBoolValue",
    "CRndrttrbts_GetBoolValue",
    "CRndrttrbts_DeleteBoolValue",
    "CRndrttrbts_SetTextureValue",
    "CRndrttrbts_GetTextureValue",
    "CRndrttrbts_DeleteTextureValue",
    "CRndrttrbts_SetSamplerValue",
    "CRndrttrbts_SetBufferValue",
    "CRndrttrbts_SetGlobalBindlessDescriptorSet",
    "CRndrttrbts_SetPtrValue",
    "CRndrttrbts_DeletePtrValue",
    "CRndrttrbts_SetIntVector4DValue",
    "CRndrttrbts_MergeToPtr",
    "CRndrttrbts_IsEmpty",
    "CRndrttrbts_AreEqual",
    "CRndrttrbts_SetParent",
    "CRndrttrbts_Clear",
    "CRenderMesh_DestroyStrongHandle",
    "CRenderMesh_IsStrongHandleValid",
    "CRenderMesh_IsError",
    "CRenderMesh_IsStrongHandleLoaded",
    "CRenderMesh_CopyStrongHandle",
    "CRenderMesh_GetBindingPtr",
    "CRenderMesh_GetGuid",
    "CRndrPplnStp_AddVolumetricFog",
    "CRndrPplnStp_Add3DSkybox",
    "CRndrPplnStp_AddTemporalUpscale",
    "CRndrPplnStp_AddAutoExposure",
    "CRndrPplnStp_SortByRenderOrder",
    "CRndrPplnStp_RendersTag",
    "CRndrPplnStp_GetShadingMode",
    "Get__CRndrPplnStp_m_pView",
    "Set__CRndrPplnStp_m_pView",
    "Get__CRndrPplnStp_m_pAttributes",
    "Set__CRndrPplnStp_m_pAttributes",
    "Get__CRndrPplnStp_m_viewport",
    "Set__CRndrPplnStp_m_viewport",
    "Get__CRndrPplnStp_m_skyViewport",
    "Set__CRndrPplnStp_m_skyViewport",
    "Get__CRndrPplnStp_m_screenViewport",
    "Set__CRndrPplnStp_m_screenViewport",
    "Get__CRndrPplnStp_m_hColor",
    "Set__CRndrPplnStp_m_hColor",
    "Get__CRndrPplnStp_m_hDepth",
    "Set__CRndrPplnStp_m_hDepth",
    "Get__CRndrPplnStp_m_hFinalColor",
    "Set__CRndrPplnStp_m_hFinalColor",
    "Get__CRndrPplnStp_m_hFinalDepth",
    "Set__CRndrPplnStp_m_hFinalDepth",
    "Get__CRndrPplnStp_m_nWidth",
    "Set__CRndrPplnStp_m_nWidth",
    "Get__CRndrPplnStp_m_nHeight",
    "Set__CRndrPplnStp_m_nHeight",
    "Get__CRndrPplnStp_m_nOutputMSAA",
    "Set__CRndrPplnStp_m_nOutputMSAA",
    "Get__CRndrPplnStp_m_nDepthFormat",
    "Set__CRndrPplnStp_m_nDepthFormat",
    "Get__CRndrPplnStp_m_hRefractColor",
    "Set__CRndrPplnStp_m_hRefractColor",
    "Get__CRndrPplnStp_m_nMSAA",
    "Set__CRndrPplnStp_m_nMSAA",
    "Get__CRndrPplnStp_m_colorSpace",
    "Set__CRndrPplnStp_m_colorSpace",
    "Get__CRndrPplnStp_m_nClearFlags",
    "Set__CRndrPplnStp_m_nClearFlags",
    "Get__CRndrPplnStp_m_vClearColor",
    "Set__CRndrPplnStp_m_vClearColor",
    "Get__CRndrPplnStp_m_bToolsView",
    "Set__CRndrPplnStp_m_bToolsView",
    "Get__CRndrPplnStp_m_bRenderSky",
    "Set__CRndrPplnStp_m_bRenderSky",
    "Get__CRndrPplnStp_m_bRenderOpaque",
    "Set__CRndrPplnStp_m_bRenderOpaque",
    "Get__CRndrPplnStp_m_bRenderDynamicObjects",
    "Set__CRndrPplnStp_m_bRenderDynamicObjects",
    "Get__CRndrPplnStp_m_bDrawOverlays",
    "Set__CRndrPplnStp_m_bDrawOverlays",
    "Get__CRndrPplnStp_m_bRenderDecals",
    "Set__CRndrPplnStp_m_bRenderDecals",
    "Get__CRndrPplnStp_m_bRenderTranslucent",
    "Set__CRndrPplnStp_m_bRenderTranslucent",
    "Get__CRndrPplnStp_m_bWireframe",
    "Set__CRndrPplnStp_m_bWireframe",
    "Get__CRndrPplnStp_m_bResolveToFinal",
    "Set__CRndrPplnStp_m_bResolveToFinal",
    "Get__CRndrPplnStp_m_bManagedHooks",
    "Set__CRndrPplnStp_m_bManagedHooks",
    "Get__CRndrPplnStp_m_bDepthPrepass",
    "Set__CRndrPplnStp_m_bDepthPrepass",
    "Get__CRndrPplnStp_m_flDepthPrepassCullThreshold",
    "Set__CRndrPplnStp_m_flDepthPrepassCullThreshold",
    "Get__CRndrPplnStp_m_nUpscalerMode",
    "Set__CRndrPplnStp_m_nUpscalerMode",
    "Get__CRndrPplnStp_m_upscaleViewport",
    "Set__CRndrPplnStp_m_upscaleViewport",
    "Get__CRndrPplnStp_m_nUpscaleRenderWidth",
    "Set__CRndrPplnStp_m_nUpscaleRenderWidth",
    "Get__CRndrPplnStp_m_nUpscaleRenderHeight",
    "Set__CRndrPplnStp_m_nUpscaleRenderHeight",
    "Get__CRndrPplnStp_m_hUpscaleColor",
    "Set__CRndrPplnStp_m_hUpscaleColor",
    "Get__CRndrPplnStp_m_hUpscaleDepth",
    "Set__CRndrPplnStp_m_hUpscaleDepth",
    "Get__CRndrPplnStp_m_hFsr1IntermediateColor",
    "Set__CRndrPplnStp_m_hFsr1IntermediateColor",
    "Get__CRndrPplnStp_m_flFsr1Sharpness",
    "Set__CRndrPplnStp_m_flFsr1Sharpness",
    "CReverbEffect_Create",
    "CReverbEffect_Dispose",
    "CReverbEffect_Apply",
    "From_CSceneObject_To_CSceneAnimatableObject",
    "To_CSceneObject_From_CSceneAnimatableObject",
    "CScnnmtblbjct_SetDeformationVolumes",
    "CScnnmtblbjct_SetDeformationAnchors",
    "CScnnmtblbjct_SetWorldSpaceRenderBoneTransform",
    "CScnnmtblbjct_GetWorldSpaceRenderBoneTransform",
    "CScnnmtblbjct_GetWorldSpaceRenderBoneTransform_1",
    "CScnnmtblbjct_GetWorldSpaceRenderBonePreviousTransform",
    "CScnnmtblbjct_GetWorldSpaceRenderBonePreviousTransform_1",
    "CScnnmtblbjct_GetLocalSpaceRenderBoneTransform",
    "CScnnmtblbjct_GetLocalSpaceRenderBoneTransform_1",
    "CScnnmtblbjct_GetWorldSpaceAnimationTransform",
    "CScnnmtblbjct_GetWorldSpaceRenderBoneTransforms",
    "CScnnmtblbjct_GetWorldSpaceRenderBonePreviousTransforms",
    "CScnnmtblbjct_GetClosestBoneIndex",
    "CScnnmtblbjct_Update",
    "CScnnmtblbjct_MergeFrom",
    "CScnnmtblbjct_SetBindPose",
    "CScnnmtblbjct_CalculateWorldSpaceBones",
    "CScnnmtblbjct_FinishUpdate",
    "CScnnmtblbjct_ResetGraphParameters",
    "CScnnmtblbjct_GetParentSpaceBone",
    "CScnnmtblbjct_GetParentSpaceBones",
    "CScnnmtblbjct_SetParentSpaceBone",
    "CScnnmtblbjct_InitAnimGraph",
    "CScnnmtblbjct_SetAnimGraph",
    "CScnnmtblbjct_SetAnimGraph_1",
    "CScnnmtblbjct_GetAnimGraph",
    "CScnnmtblbjct_SBox_SetFlexOverride",
    "CScnnmtblbjct_SBox_SetFlexOverride_1",
    "CScnnmtblbjct_SBox_GetFlexOverride",
    "CScnnmtblbjct_SBox_GetFlexOverride_1",
    "CScnnmtblbjct_SBox_ClearFlexOverride",
    "CScnnmtblbjct_SBox_ClearFlexOverride_1",
    "CScnnmtblbjct_SBox_ClearFlexOverride_2",
    "CScnnmtblbjct_DirectPlayback_PlaySequence",
    "CScnnmtblbjct_DirectPlayback_PlaySequence_1",
    "CScnnmtblbjct_DirectPlayback_CancelSequence",
    "CScnnmtblbjct_DirectPlayback_GetSequenceCycle",
    "CScnnmtblbjct_DirectPlayback_GetSequence",
    "CScnnmtblbjct_DirectPlayback_SetSequenceStartTime",
    "CScnnmtblbjct_DirectPlayback_GetSequenceDuration",
    "CScnnmtblbjct_SBox_GetAttachment",
    "CScnnmtblbjct_SetShouldUseAnimGraph",
    "CScnnmtblbjct_GetShouldUseAnimGraph",
    "CScnnmtblbjct_GetSequence",
    "CScnnmtblbjct_SetSequence",
    "CScnnmtblbjct_GetSequenceDuration",
    "CScnnmtblbjct_GetSequenceCycle",
    "CScnnmtblbjct_SetSequenceCycle",
    "CScnnmtblbjct_SetSequenceLooping",
    "CScnnmtblbjct_IsSequenceFinished",
    "CScnnmtblbjct_SetSequenceBlending",
    "CScnnmtblbjct_GetPlaybackRate",
    "CScnnmtblbjct_SetPlaybackRate",
    "CScnnmtblbjct_PendingAnimationEvents",
    "CScnnmtblbjct_RunAnimationEvents",
    "CScnnmtblbjct_DispatchTagEvents",
    "CScnnmtblbjct_ClearPhysicsBones",
    "CScnnmtblbjct_SetPhysicsBone",
    "CScnnmtblbjct_GetRootMotion",
    "CScnnmtblbjct_HasPhysicsBones",
    "CScnnmtblbjct_GetAnimParameter",
    "CScnnmtblbjct_GetAnimParameter_1",
    "CScnnmtblbjct_ChangeFlags",
    "CScnnmtblbjct_SetFlags",
    "CScnnmtblbjct_HasFlags",
    "CScnnmtblbjct_GetFlags",
    "CScnnmtblbjct_GetOriginalFlags",
    "CScnnmtblbjct_ClearFlags",
    "CScnnmtblbjct_SetCullDistance",
    "CScnnmtblbjct_EnableLightingCache",
    "CScnnmtblbjct_SetLightingOrigin",
    "CScnnmtblbjct_GetLightingOrigin",
    "CScnnmtblbjct_HasLightingOrigin",
    "CScnnmtblbjct_SetTintRGBA",
    "CScnnmtblbjct_GetTintRGBA",
    "CScnnmtblbjct_SetAlphaFade",
    "CScnnmtblbjct_GetAlphaFade",
    "CScnnmtblbjct_SetMaterialOverrideForMeshInstances",
    "CScnnmtblbjct_ClearMaterialOverrideList",
    "CScnnmtblbjct_SetMaterialOverride",
    "CScnnmtblbjct_IsLoaded",
    "CScnnmtblbjct_IsRenderingEnabled",
    "CScnnmtblbjct_SetLoaded",
    "CScnnmtblbjct_ClearLoaded",
    "CScnnmtblbjct_DisableRendering",
    "CScnnmtblbjct_EnableRendering",
    "CScnnmtblbjct_SetRenderingEnabled",
    "CScnnmtblbjct_GetBoundingSphereRadius",
    "CScnnmtblbjct_SetTransform",
    "CScnnmtblbjct_GetCTransform",
    "CScnnmtblbjct_SetBounds",
    "CScnnmtblbjct_GetBounds",
    "CScnnmtblbjct_SetBoundsInfinite",
    "CScnnmtblbjct_GetParent",
    "CScnnmtblbjct_AddChildObject",
    "CScnnmtblbjct_RemoveChild",
    "CScnnmtblbjct_GetAttributesPtrForModify",
    "CScnnmtblbjct_EnableMeshGroups",
    "CScnnmtblbjct_DisableMeshGroups",
    "CScnnmtblbjct_ResetMeshGroups",
    "CScnnmtblbjct_GetCurrentMeshGroupMask",
    "CScnnmtblbjct_GetWorld",
    "CScnnmtblbjct_SetLOD",
    "CScnnmtblbjct_DisableLOD",
    "CScnnmtblbjct_GetCurrentLODGroupMask",
    "CScnnmtblbjct_GetCurrentLODLevel",
    "CScnnmtblbjct_GetModelHandle",
    "CScnnmtblbjct_SetMaterialGroup",
    "CScnnmtblbjct_SetBodyGroup",
    "CScnnmtblbjct_SetBatchable",
    "CScnnmtblbjct_IsNotBatchable",
    "CScnnmtblbjct_SetUniqueBatchGroup",
    "CScnnmtblbjct_RemoveTag",
    "CScnnmtblbjct_RemoveAllTags",
    "CScnnmtblbjct_GetTagCount",
    "CScnnmtblbjct_GetTagAt",
    "CScnnmtblbjct_AddTag",
    "CScnnmtblbjct_HasTag",
    "CScnnmtblbjct_SetForceLayerID",
    "CScnnmtblbjct_SetLayerMatchID",
    "CScnnmtblbjct_GetLayerMatchIDValue",
    "CScnnmtblbjct_GetIntValue",
    "CScnnmtblbjct_UpdateFlagsBasedOnMaterial",
    "CScnnmtblbjct_SetMaterialOverrideByIndex",
    "Get__CScnnmtblbjct_m_flDeltaTime",
    "Set__CScnnmtblbjct_m_flDeltaTime",
    "Get__CScnnmtblbjct_m_worldBounds",
    "Set__CScnnmtblbjct_m_worldBounds",
    "Get__CScnnmtblbjct_m_localBounds",
    "Set__CScnnmtblbjct_m_localBounds",
    "From_CSceneObject_To_CSceneLightObject",
    "To_CSceneObject_From_CSceneLightObject",
    "CScnLghtbjct_SetWorldPosition",
    "CScnLghtbjct_GetWorldPosition",
    "CScnLghtbjct_SetWorldDirection",
    "CScnLghtbjct_GetWorldDirection",
    "CScnLghtbjct_SetColor",
    "CScnLghtbjct_SetBounceColor",
    "CScnLghtbjct_GetColor",
    "CScnLghtbjct_SetRadius",
    "CScnLghtbjct_GetRadius",
    "CScnLghtbjct_SetTheta",
    "CScnLghtbjct_GetTheta",
    "CScnLghtbjct_SetPhi",
    "CScnLghtbjct_GetPhi",
    "CScnLghtbjct_SetFallOff",
    "CScnLghtbjct_GetFallOff",
    "CScnLghtbjct_GetShadowTextureResolution",
    "CScnLghtbjct_SetShadowTextureResolution",
    "CScnLghtbjct_GetShadows",
    "CScnLghtbjct_SetShadows",
    "CScnLghtbjct_SetConstantAttn",
    "CScnLghtbjct_GetConstantAttn",
    "CScnLghtbjct_SetLinearAttn",
    "CScnLghtbjct_GetLinearAttn",
    "CScnLghtbjct_SetQuadraticAttn",
    "CScnLghtbjct_GetQuadraticAttn",
    "CScnLghtbjct_SetLightCookie",
    "CScnLghtbjct_GetLightCookie",
    "CScnLghtbjct_SetLightModulatorRange",
    "CScnLghtbjct_GetShadowCascades",
    "CScnLghtbjct_SetShadowCascades",
    "CScnLghtbjct_GetShadowCascadeSplitRatio",
    "CScnLghtbjct_SetShadowCascadeSplitRatio",
    "CScnLghtbjct_GetCascadeDistanceScale",
    "CScnLghtbjct_SetCascadeDistanceScale",
    "CScnLghtbjct_GetFogContributionStength",
    "CScnLghtbjct_SetFogContributionStength",
    "CScnLghtbjct_GetFogLightingMode",
    "CScnLghtbjct_SetFogLightingMode",
    "CScnLghtbjct_SetBakeLightIndex",
    "CScnLghtbjct_SetBakeLightIndexScale",
    "CScnLghtbjct_SetUsesIndexedBakedLighting",
    "CScnLghtbjct_SetRenderDiffuse",
    "CScnLghtbjct_SetRenderSpecular",
    "CScnLghtbjct_SetRenderTransmissive",
    "CScnLghtbjct_SetLightSourceSize0",
    "CScnLghtbjct_SetLightSourceSize1",
    "CScnLghtbjct_SetShadowTextureWidth",
    "CScnLghtbjct_SetShadowTextureHeight",
    "CScnLghtbjct_GetShadowTextureWidth",
    "CScnLghtbjct_GetShadowTextureHeight",
    "CScnLghtbjct_GetLightFlags",
    "CScnLghtbjct_SetLightFlags",
    "CScnLghtbjct_GetLightShape",
    "CScnLghtbjct_SetLightShape",
    "CScnLghtbjct_SetLightSourceDim0",
    "CScnLghtbjct_SetLightSourceDim1",
    "CScnLghtbjct_GetLightType",
    "CScnLghtbjct_ChangeFlags",
    "CScnLghtbjct_SetFlags",
    "CScnLghtbjct_HasFlags",
    "CScnLghtbjct_GetFlags",
    "CScnLghtbjct_GetOriginalFlags",
    "CScnLghtbjct_ClearFlags",
    "CScnLghtbjct_SetCullDistance",
    "CScnLghtbjct_EnableLightingCache",
    "CScnLghtbjct_SetLightingOrigin",
    "CScnLghtbjct_GetLightingOrigin",
    "CScnLghtbjct_HasLightingOrigin",
    "CScnLghtbjct_SetTintRGBA",
    "CScnLghtbjct_GetTintRGBA",
    "CScnLghtbjct_SetAlphaFade",
    "CScnLghtbjct_GetAlphaFade",
    "CScnLghtbjct_SetMaterialOverrideForMeshInstances",
    "CScnLghtbjct_ClearMaterialOverrideList",
    "CScnLghtbjct_SetMaterialOverride",
    "CScnLghtbjct_IsLoaded",
    "CScnLghtbjct_IsRenderingEnabled",
    "CScnLghtbjct_SetLoaded",
    "CScnLghtbjct_ClearLoaded",
    "CScnLghtbjct_DisableRendering",
    "CScnLghtbjct_EnableRendering",
    "CScnLghtbjct_SetRenderingEnabled",
    "CScnLghtbjct_GetBoundingSphereRadius",
    "CScnLghtbjct_SetTransform",
    "CScnLghtbjct_GetCTransform",
    "CScnLghtbjct_SetBounds",
    "CScnLghtbjct_GetBounds",
    "CScnLghtbjct_SetBoundsInfinite",
    "CScnLghtbjct_GetParent",
    "CScnLghtbjct_AddChildObject",
    "CScnLghtbjct_RemoveChild",
    "CScnLghtbjct_GetAttributesPtrForModify",
    "CScnLghtbjct_EnableMeshGroups",
    "CScnLghtbjct_DisableMeshGroups",
    "CScnLghtbjct_ResetMeshGroups",
    "CScnLghtbjct_GetCurrentMeshGroupMask",
    "CScnLghtbjct_GetWorld",
    "CScnLghtbjct_SetLOD",
    "CScnLghtbjct_DisableLOD",
    "CScnLghtbjct_GetCurrentLODGroupMask",
    "CScnLghtbjct_GetCurrentLODLevel",
    "CScnLghtbjct_GetModelHandle",
    "CScnLghtbjct_SetMaterialGroup",
    "CScnLghtbjct_SetBodyGroup",
    "CScnLghtbjct_SetBatchable",
    "CScnLghtbjct_IsNotBatchable",
    "CScnLghtbjct_SetUniqueBatchGroup",
    "CScnLghtbjct_RemoveTag",
    "CScnLghtbjct_RemoveAllTags",
    "CScnLghtbjct_GetTagCount",
    "CScnLghtbjct_GetTagAt",
    "CScnLghtbjct_AddTag",
    "CScnLghtbjct_HasTag",
    "CScnLghtbjct_SetForceLayerID",
    "CScnLghtbjct_SetLayerMatchID",
    "CScnLghtbjct_GetLayerMatchIDValue",
    "CScnLghtbjct_GetIntValue",
    "CScnLghtbjct_UpdateFlagsBasedOnMaterial",
    "CScnLghtbjct_SetMaterialOverrideByIndex",
    "From_CSceneObject_To_CSceneLightProbeVolumeObject",
    "To_CSceneObject_From_CSceneLightProbeVolumeObject",
    "CScnLghtPrbVlmbj_CreateConstants",
    "CScnLghtPrbVlmbj_ChangeFlags",
    "CScnLghtPrbVlmbj_SetFlags",
    "CScnLghtPrbVlmbj_HasFlags",
    "CScnLghtPrbVlmbj_GetFlags",
    "CScnLghtPrbVlmbj_GetOriginalFlags",
    "CScnLghtPrbVlmbj_ClearFlags",
    "CScnLghtPrbVlmbj_SetCullDistance",
    "CScnLghtPrbVlmbj_EnableLightingCache",
    "CScnLghtPrbVlmbj_SetLightingOrigin",
    "CScnLghtPrbVlmbj_GetLightingOrigin",
    "CScnLghtPrbVlmbj_HasLightingOrigin",
    "CScnLghtPrbVlmbj_SetTintRGBA",
    "CScnLghtPrbVlmbj_GetTintRGBA",
    "CScnLghtPrbVlmbj_SetAlphaFade",
    "CScnLghtPrbVlmbj_GetAlphaFade",
    "CScnLghtPrbVlmbj_SetMaterialOverrideForMeshInstances",
    "CScnLghtPrbVlmbj_ClearMaterialOverrideList",
    "CScnLghtPrbVlmbj_SetMaterialOverride",
    "CScnLghtPrbVlmbj_IsLoaded",
    "CScnLghtPrbVlmbj_IsRenderingEnabled",
    "CScnLghtPrbVlmbj_SetLoaded",
    "CScnLghtPrbVlmbj_ClearLoaded",
    "CScnLghtPrbVlmbj_DisableRendering",
    "CScnLghtPrbVlmbj_EnableRendering",
    "CScnLghtPrbVlmbj_SetRenderingEnabled",
    "CScnLghtPrbVlmbj_GetBoundingSphereRadius",
    "CScnLghtPrbVlmbj_SetTransform",
    "CScnLghtPrbVlmbj_GetCTransform",
    "CScnLghtPrbVlmbj_SetBounds",
    "CScnLghtPrbVlmbj_GetBounds",
    "CScnLghtPrbVlmbj_SetBoundsInfinite",
    "CScnLghtPrbVlmbj_GetParent",
    "CScnLghtPrbVlmbj_AddChildObject",
    "CScnLghtPrbVlmbj_RemoveChild",
    "CScnLghtPrbVlmbj_GetAttributesPtrForModify",
    "CScnLghtPrbVlmbj_EnableMeshGroups",
    "CScnLghtPrbVlmbj_DisableMeshGroups",
    "CScnLghtPrbVlmbj_ResetMeshGroups",
    "CScnLghtPrbVlmbj_GetCurrentMeshGroupMask",
    "CScnLghtPrbVlmbj_GetWorld",
    "CScnLghtPrbVlmbj_SetLOD",
    "CScnLghtPrbVlmbj_DisableLOD",
    "CScnLghtPrbVlmbj_GetCurrentLODGroupMask",
    "CScnLghtPrbVlmbj_GetCurrentLODLevel",
    "CScnLghtPrbVlmbj_GetModelHandle",
    "CScnLghtPrbVlmbj_SetMaterialGroup",
    "CScnLghtPrbVlmbj_SetBodyGroup",
    "CScnLghtPrbVlmbj_SetBatchable",
    "CScnLghtPrbVlmbj_IsNotBatchable",
    "CScnLghtPrbVlmbj_SetUniqueBatchGroup",
    "CScnLghtPrbVlmbj_RemoveTag",
    "CScnLghtPrbVlmbj_RemoveAllTags",
    "CScnLghtPrbVlmbj_GetTagCount",
    "CScnLghtPrbVlmbj_GetTagAt",
    "CScnLghtPrbVlmbj_AddTag",
    "CScnLghtPrbVlmbj_HasTag",
    "CScnLghtPrbVlmbj_SetForceLayerID",
    "CScnLghtPrbVlmbj_SetLayerMatchID",
    "CScnLghtPrbVlmbj_GetLayerMatchIDValue",
    "CScnLghtPrbVlmbj_GetIntValue",
    "CScnLghtPrbVlmbj_UpdateFlagsBasedOnMaterial",
    "CScnLghtPrbVlmbj_SetMaterialOverrideByIndex",
    "Get__CScnLghtPrbVlmbj_m_vBoxMins",
    "Set__CScnLghtPrbVlmbj_m_vBoxMins",
    "Get__CScnLghtPrbVlmbj_m_vBoxMaxs",
    "Set__CScnLghtPrbVlmbj_m_vBoxMaxs",
    "Get__CScnLghtPrbVlmbj_m_nHandshake",
    "Set__CScnLghtPrbVlmbj_m_nHandshake",
    "Get__CScnLghtPrbVlmbj_m_nRenderPriority",
    "Set__CScnLghtPrbVlmbj_m_nRenderPriority",
    "Get__CScnLghtPrbVlmbj_m_hLightProbeTexture",
    "Set__CScnLghtPrbVlmbj_m_hLightProbeTexture",
    "Get__CScnLghtPrbVlmbj_m_hLightProbeDirectLightIndicesTexture",
    "Set__CScnLghtPrbVlmbj_m_hLightProbeDirectLightIndicesTexture",
    "Get__CScnLghtPrbVlmbj_m_hLightProbeDirectLightScalarsTexture",
    "Set__CScnLghtPrbVlmbj_m_hLightProbeDirectLightScalarsTexture",
    "CSceneObject_ChangeFlags",
    "CSceneObject_SetFlags",
    "CSceneObject_HasFlags",
    "CSceneObject_GetFlags",
    "CSceneObject_GetOriginalFlags",
    "CSceneObject_ClearFlags",
    "CSceneObject_SetCullDistance",
    "CSceneObject_EnableLightingCache",
    "CSceneObject_SetLightingOrigin",
    "CSceneObject_GetLightingOrigin",
    "CSceneObject_HasLightingOrigin",
    "CSceneObject_SetTintRGBA",
    "CSceneObject_GetTintRGBA",
    "CSceneObject_SetAlphaFade",
    "CSceneObject_GetAlphaFade",
    "CSceneObject_SetMaterialOverrideForMeshInstances",
    "CSceneObject_ClearMaterialOverrideList",
    "CSceneObject_SetMaterialOverride",
    "CSceneObject_IsLoaded",
    "CSceneObject_IsRenderingEnabled",
    "CSceneObject_SetLoaded",
    "CSceneObject_ClearLoaded",
    "CSceneObject_DisableRendering",
    "CSceneObject_EnableRendering",
    "CSceneObject_SetRenderingEnabled",
    "CSceneObject_GetBoundingSphereRadius",
    "CSceneObject_SetTransform",
    "CSceneObject_GetCTransform",
    "CSceneObject_SetBounds",
    "CSceneObject_GetBounds",
    "CSceneObject_SetBoundsInfinite",
    "CSceneObject_GetParent",
    "CSceneObject_AddChildObject",
    "CSceneObject_RemoveChild",
    "CSceneObject_GetAttributesPtrForModify",
    "CSceneObject_EnableMeshGroups",
    "CSceneObject_DisableMeshGroups",
    "CSceneObject_ResetMeshGroups",
    "CSceneObject_GetCurrentMeshGroupMask",
    "CSceneObject_GetWorld",
    "CSceneObject_SetLOD",
    "CSceneObject_DisableLOD",
    "CSceneObject_GetCurrentLODGroupMask",
    "CSceneObject_GetCurrentLODLevel",
    "CSceneObject_GetModelHandle",
    "CSceneObject_SetMaterialGroup",
    "CSceneObject_SetBodyGroup",
    "CSceneObject_SetBatchable",
    "CSceneObject_IsNotBatchable",
    "CSceneObject_SetUniqueBatchGroup",
    "CSceneObject_RemoveTag",
    "CSceneObject_RemoveAllTags",
    "CSceneObject_GetTagCount",
    "CSceneObject_GetTagAt",
    "CSceneObject_AddTag",
    "CSceneObject_HasTag",
    "CSceneObject_SetForceLayerID",
    "CSceneObject_SetLayerMatchID",
    "CSceneObject_GetLayerMatchIDValue",
    "CSceneObject_GetIntValue",
    "CSceneObject_UpdateFlagsBasedOnMaterial",
    "CSceneObject_SetMaterialOverrideByIndex",
    "From_CSceneObject_To_CSceneSkyBoxObject",
    "To_CSceneObject_From_CSceneSkyBoxObject",
    "CScnSkyBxbjct_SetLighting_ConstantColorHemisphere",
    "CScnSkyBxbjct_SetLighting_Samples",
    "CScnSkyBxbjct_GetMaterial",
    "CScnSkyBxbjct_SetMaterial",
    "CScnSkyBxbjct_SetSkyTint",
    "CScnSkyBxbjct_GetSkyTint",
    "CScnSkyBxbjct_SetFogType",
    "CScnSkyBxbjct_GetFogType",
    "CScnSkyBxbjct_SetAngularFogParams",
    "CScnSkyBxbjct_GetFogMinStart",
    "CScnSkyBxbjct_GetFogMinEnd",
    "CScnSkyBxbjct_GetFogMaxStart",
    "CScnSkyBxbjct_GetFogMaxEnd",
    "CScnSkyBxbjct_ChangeFlags",
    "CScnSkyBxbjct_SetFlags",
    "CScnSkyBxbjct_HasFlags",
    "CScnSkyBxbjct_GetFlags",
    "CScnSkyBxbjct_GetOriginalFlags",
    "CScnSkyBxbjct_ClearFlags",
    "CScnSkyBxbjct_SetCullDistance",
    "CScnSkyBxbjct_EnableLightingCache",
    "CScnSkyBxbjct_SetLightingOrigin",
    "CScnSkyBxbjct_GetLightingOrigin",
    "CScnSkyBxbjct_HasLightingOrigin",
    "CScnSkyBxbjct_SetTintRGBA",
    "CScnSkyBxbjct_GetTintRGBA",
    "CScnSkyBxbjct_SetAlphaFade",
    "CScnSkyBxbjct_GetAlphaFade",
    "CScnSkyBxbjct_SetMaterialOverrideForMeshInstances",
    "CScnSkyBxbjct_ClearMaterialOverrideList",
    "CScnSkyBxbjct_SetMaterialOverride",
    "CScnSkyBxbjct_IsLoaded",
    "CScnSkyBxbjct_IsRenderingEnabled",
    "CScnSkyBxbjct_SetLoaded",
    "CScnSkyBxbjct_ClearLoaded",
    "CScnSkyBxbjct_DisableRendering",
    "CScnSkyBxbjct_EnableRendering",
    "CScnSkyBxbjct_SetRenderingEnabled",
    "CScnSkyBxbjct_GetBoundingSphereRadius",
    "CScnSkyBxbjct_SetTransform",
    "CScnSkyBxbjct_GetCTransform",
    "CScnSkyBxbjct_SetBounds",
    "CScnSkyBxbjct_GetBounds",
    "CScnSkyBxbjct_SetBoundsInfinite",
    "CScnSkyBxbjct_GetParent",
    "CScnSkyBxbjct_AddChildObject",
    "CScnSkyBxbjct_RemoveChild",
    "CScnSkyBxbjct_GetAttributesPtrForModify",
    "CScnSkyBxbjct_EnableMeshGroups",
    "CScnSkyBxbjct_DisableMeshGroups",
    "CScnSkyBxbjct_ResetMeshGroups",
    "CScnSkyBxbjct_GetCurrentMeshGroupMask",
    "CScnSkyBxbjct_GetWorld",
    "CScnSkyBxbjct_SetLOD",
    "CScnSkyBxbjct_DisableLOD",
    "CScnSkyBxbjct_GetCurrentLODGroupMask",
    "CScnSkyBxbjct_GetCurrentLODLevel",
    "CScnSkyBxbjct_GetModelHandle",
    "CScnSkyBxbjct_SetMaterialGroup",
    "CScnSkyBxbjct_SetBodyGroup",
    "CScnSkyBxbjct_SetBatchable",
    "CScnSkyBxbjct_IsNotBatchable",
    "CScnSkyBxbjct_SetUniqueBatchGroup",
    "CScnSkyBxbjct_RemoveTag",
    "CScnSkyBxbjct_RemoveAllTags",
    "CScnSkyBxbjct_GetTagCount",
    "CScnSkyBxbjct_GetTagAt",
    "CScnSkyBxbjct_AddTag",
    "CScnSkyBxbjct_HasTag",
    "CScnSkyBxbjct_SetForceLayerID",
    "CScnSkyBxbjct_SetLayerMatchID",
    "CScnSkyBxbjct_GetLayerMatchIDValue",
    "CScnSkyBxbjct_GetIntValue",
    "CScnSkyBxbjct_UpdateFlagsBasedOnMaterial",
    "CScnSkyBxbjct_SetMaterialOverrideByIndex",
    "CServerList_Create",
    "CServerList_Destroy",
    "CServerList_StartQuery",
    "CServerList_AddFilter",
    "CSfxTable_GetCacheStatus",
    "CSfxTable_GetSampleCount",
    "CSfxTable_GetSamples",
    "CSfxTable_GetSamplesInterleaved",
    "CSfxTable_GetSound",
    "CSfxTable_IsValidForPlayback",
    "CSfxTable_FailedResourceLoad",
    "CSfxTable_CreateMixer",
    "CSmplPCMWvDt_Create",
    "CSmplPCMWvDt_Dispose",
    "CSmplPCMWvDt_ParseMP3File",
    "CSmplPCMWvDt_ParseOGGFile",
    "CSmplPCMWvDt_GetPCMData",
    "CSmplPCMWvDt_GetPCMSize",
    "Get__CSmplPCMWvDt_m_nFormat",
    "Set__CSmplPCMWvDt_m_nFormat",
    "Get__CSmplPCMWvDt_m_nSampleRate",
    "Set__CSmplPCMWvDt_m_nSampleRate",
    "Get__CSmplPCMWvDt_m_nChannels",
    "Set__CSmplPCMWvDt_m_nChannels",
    "Get__CSmplPCMWvDt_m_nSampleCount",
    "Set__CSmplPCMWvDt_m_nSampleCount",
    "Get__CSmplPCMWvDt_m_nBits",
    "Set__CSmplPCMWvDt_m_nBits",
    "Get__CSmplPCMWvDt_m_flDuration",
    "Set__CSmplPCMWvDt_m_flDuration",
    "CStmnvntryRslt_Destroy",
    "CStmnvntryRslt_IsPending",
    "CStmnvntryRslt_IsOk",
    "CStmnvntryRslt_CheckSteamId",
    "CStmnvntryRslt_GetTimestamp",
    "CStmnvntryRslt_Count",
    "CStmnvntryRslt_Get",
    "CStmnvntryRslt_GetSerializedSize",
    "CStmnvntryRslt_Serialize",
    "CStmtmnstnc_ItemId",
    "CStmtmnstnc_DefinitionId",
    "CTextureBase_DestroyStrongHandle",
    "CTextureBase_IsStrongHandleValid",
    "CTextureBase_IsError",
    "CTextureBase_IsStrongHandleLoaded",
    "CTextureBase_CopyStrongHandle",
    "CTextureBase_GetBindingPtr",
    "CTextureBase_GetGuid",
    "CUtlBuffer_Create",
    "CUtlBuffer_Dispose",
    "CUtlBuffer_Base",
    "CUtlBuffer_TellMaxPut",
    "CUtlSymbolTable_AddString",
    "CtlVctrCtlStrng_DeleteThis",
    "CtlVctrCtlStrng_Create",
    "CtlVctrCtlStrng_Count",
    "CtlVctrCtlStrng_SetCount",
    "CtlVctrCtlStrng_Element",
    "CtlVctrHRndrTxtr_DeleteThis",
    "CtlVctrHRndrTxtr_Create",
    "CtlVctrHRndrTxtr_Count",
    "CtlVctrHRndrTxtr_Element",
    "CtlVctrMshTrctpt_DeleteThis",
    "CtlVctrMshTrctpt_Create",
    "CtlVctrMshTrctpt_Count",
    "CtlVctrMshTrctpt_Element",
    "CtlVctrPhyscsTrc_Result_DeleteThis",
    "CtlVctrPhyscsTrc_Result_Create",
    "CtlVctrPhyscsTrc_Result_Count",
    "CtlVctrPhyscsTrc_Result_Element",
    "CtlVctrPhyscsTrc_Result_RemoveAll",
    "CtlVctrVctr_DeleteThis",
    "CtlVctrVctr_Create",
    "CtlVctrVctr_Count",
    "CtlVctrVctr_SetCount",
    "CtlVctrVctr_Element",
    "CtlVctrflt_DeleteThis",
    "CtlVctrflt_Create",
    "CtlVctrflt_Count",
    "CtlVctrflt_SetCount",
    "CtlVctrflt_Element",
    "CtlVctrnt32_DeleteThis",
    "CtlVctrnt32_Create",
    "CtlVctrnt32_Count",
    "CtlVctrnt32_SetCount",
    "CtlVctrnt32_Element",
    "CVfx_DestroyStrongHandle",
    "CVfx_IsStrongHandleValid",
    "CVfx_IsError",
    "CVfx_IsStrongHandleLoaded",
    "CVfx_CopyStrongHandle",
    "CVfx_GetBindingPtr",
    "CVfx_GetGuid",
    "CVfx_Create",
    "CVfx_CreateAnonymous",
    "CVfx_GetFilename",
    "CVfx_CreateFromResourceFile",
    "CVfx_CreateFromShaderFile",
    "CVfx_GetProgramData",
    "CVfx_GetIterator",
    "CVfx_Serialize",
    "CVfx_HasShaderProgram",
    "CVfx_InitializeWrite",
    "CVfx_FinalizeCompile",
    "CVfx_WriteProgramToBuffer",
    "CVfx_WriteCombo",
    "CVfx_GetPropertiesJson",
    "CVfx_DescribeCombo",
    "CVfx_IsUiVariableActive",
    "CVfxBytCdMngr_Create",
    "CVfxBytCdMngr_Delete",
    "CVfxBytCdMngr_OnStaticCombo",
    "CVfxBytCdMngr_OnDynamicCombo",
    "CVfxBytCdMngr_Reset",
    "CVfxCombo_GetName",
    "CVfxCombo_GetGroup",
    "Get__CVfxCombo_m_nMin",
    "Set__CVfxCombo_m_nMin",
    "Get__CVfxCombo_m_nMax",
    "Set__CVfxCombo_m_nMax",
    "CVfxCmbtrtr_Delete",
    "CVfxCmbtrtr_InvalidIndex",
    "CVfxCmbtrtr_SetStaticCombo",
    "CVfxCmbtrtr_FirstStaticCombo",
    "CVfxCmbtrtr_NextStaticCombo",
    "CVfxCmbtrtr_SetDynamicCombo",
    "CVfxCmbtrtr_FirstDynamicCombo",
    "CVfxCmbtrtr_NextDynamicCombo",
    "Get__CVfxProgramData_m_bLoadedFromVcsFile",
    "Set__CVfxProgramData_m_bLoadedFromVcsFile",
    "CVideoPlayer_Create",
    "CVideoPlayer_Destroy",
    "CVideoPlayer_Play",
    "CVideoPlayer_Resume",
    "CVideoPlayer_Stop",
    "CVideoPlayer_Pause",
    "CVideoPlayer_Seek",
    "CVideoPlayer_GetRepeat",
    "CVideoPlayer_SetRepeat",
    "CVideoPlayer_GetDuration",
    "CVideoPlayer_GetPlaybackTime",
    "CVideoPlayer_HasAudioStream",
    "CVideoPlayer_Update",
    "CVideoPlayer_IsPaused",
    "CVideoPlayer_IsMuted",
    "CVideoPlayer_SetMuted",
    "CVideoPlayer_GetWidth",
    "CVideoPlayer_GetHeight",
    "CVideoPlayer_SetVideoOnly",
    "CVideoPlayer_GetMetadata",
    "CVideoPlayer_GetSpectrum",
    "CVideoPlayer_GetAmplitude",
    "CVideoPlayer_GetVideoDecodeSeconds",
    "CVideoPlayer_GetAudioDecodeSeconds",
    "CVideoPlayer_GetDecodedFrameCount",
    "CVideoPlayer_GetPacketQueueDepth",
    "CVideoPlayer_GetFrameQueueDepth",
    "CVideoPlayer_GetTexture",
    "CVideoPlayer_GetAudioStream",
    "CVideoRecorder_Create",
    "CVideoRecorder_Destroy",
    "CVideoRecorder_Stop",
    "CVideoRecorder_ReadyForVideoFrame",
    "CVideoRecorder_AddVideoFrame",
    "CVideoRecorder_AddAudioSamples",
    "CVideoRecorder_Initialize",
    "ConCommand_GetName",
    "ConCommand_GetHelpText",
    "ConCommand_GetFlags",
    "ConCommand_Run",
    "ConVar_GetName",
    "ConVar_GetHelpText",
    "ConVar_SetValue",
    "ConVar_SetValue_1",
    "ConVar_SetValue_2",
    "ConVar_GetString",
    "ConVar_Revert",
    "ConVar_HasMin",
    "ConVar_HasMax",
    "ConVar_GetMinValue",
    "ConVar_GetMaxValue",
    "ConVar_GetDefault",
    "ConVar_GetFlags",
    "DspInstance_Dispose",
    "DspInstance_ProcessChannel",
    "DspPreset_Create",
    "DspPreset_Dispose",
    "DspPreset_AddProcessor",
    "DspPreset_FinishBuilding",
    "DspPreset_Instantiate",
    "EngineGlue_JsonToKeyValues3",
    "EngineGlue_KeyValuesToJson",
    "EngineGlue_KeyValues3ToJson",
    "EngineGlue_LoadKeyValues3",
    "EngineGlue_GetStringToken",
    "EngineGlue_GetStringTokenValue",
    "EngineGlue_AddSearchPath",
    "EngineGlue_RemoveSearchPath",
    "EngineGlue_ApproximateProcessMemoryUsage",
    "EngineGlue_ReadCompiledResourceFileJson",
    "EngineGlue_ReadCompiledResourceFileBlock",
    "EngineGlue_ReadCompiledResourceFileJsonFromFilesystem",
    "EngineGlue_SetEngineLoggingVerbose",
    "EngineGlue_RequestWebAuthTicket",
    "EngineGlue_CancelWebAuthTicket",
    "EngineGlue_GetWebAuthTicket",
    "ErrorReports_Init",
    "ErrorReports_SetTag",
    "ErrorReports_Breadcrumb",
    "ErrorReports_Shutdown",
    "FloatBitMap_t_Create",
    "FloatBitMap_t_Create_1",
    "FloatBitMap_t_Delete",
    "FloatBitMap_t_LoadFromFile",
    "FloatBitMap_t_LoadFromPFM",
    "FloatBitMap_t_LoadFromPSD",
    "FloatBitMap_t_LoadFromTIF",
    "FloatBitMap_t_LoadFromPNG",
    "FloatBitMap_t_LoadFromJPG",
    "FloatBitMap_t_LoadFromEXR",
    "FloatBitMap_t_LoadFromBuffer",
    "FloatBitMap_t_Init",
    "FloatBitMap_t_Shutdown",
    "FloatBitMap_t_SetChannel",
    "FloatBitMap_t_Rotate90DegreesCW",
    "FloatBitMap_t_Rotate90DegreesCCW",
    "FloatBitMap_t_Rotate180Degrees",
    "FloatBitMap_t_MirrorHorizontally",
    "FloatBitMap_t_MirrorVertically",
    "FloatBitMap_t_WriteTGAFile",
    "FloatBitMap_t_WritePFM",
    "FloatBitMap_t_WriteEXR",
    "FloatBitMap_t_LoadFromInMemoryTGA",
    "FloatBitMap_t_LoadFromInMemoryPSD",
    "FloatBitMap_t_LoadFromInMemoryTIF",
    "FloatBitMap_t_Pixel",
    "FloatBitMap_t_PixelWrapped",
    "FloatBitMap_t_PixelClamped",
    "FloatBitMap_t_Alpha",
    "FloatBitMap_t_RGBPixelAsVector",
    "FloatBitMap_t_Width",
    "FloatBitMap_t_Height",
    "FloatBitMap_t_Depth",
    "FloatBitMap_t_Resize2D",
    "FloatBitMap_t_WriteToBuffer",
    "GameWindowGlue_FromNativeHandle",
    "GameWindowGlue_Attach",
    "GameWindowGlue_Detach",
    "GameWindowGlue_SetRenderTarget",
    "Glue_Networking_RunCallbacks",
    "Glue_Networking_SetDebugFunction",
    "Glue_Networking_GetAuthenticationStatus",
    "Glue_Networking_GetRelayNetworkStatus",
    "Glue_Networking_CreateSocket",
    "Glue_Networking_CreateIpBasedSocket",
    "Glue_Networking_CloseSocket",
    "Glue_Networking_GetSocketAddress",
    "Glue_Networking_BeginAsyncRequestFakeIP",
    "Glue_Networking_GetIdentity",
    "Glue_Networking_CreatePollGroup",
    "Glue_Networking_DestroyPollGroup",
    "Glue_Networking_SetPollGroup",
    "Glue_Networking_GetPollGroupMessages",
    "Glue_Networking_ConnectToSteamId",
    "Glue_Networking_ConnectToIpAddress",
    "Glue_Networking_CloseConnection",
    "Glue_Networking_AcceptConnection",
    "Glue_Networking_GetMessageDataBuffer",
    "Glue_Networking_AllocateMessageWithManagedBuffer",
    "Glue_Networking_AllocateMessage",
    "Glue_Networking_FlushMessagesOnConnection",
    "Glue_Networking_SendMessage",
    "Glue_Networking_SendMessages",
    "Glue_Networking_GetConnectionMessages",
    "Glue_Networking_GetConnectionState",
    "Glue_Networking_GetConnectionDescription",
    "Glue_Networking_GetConnectionSteamId",
    "Glue_Networking_SetFreeDataCallback",
    "Glue_RndrDvcMngr_GetBestMultisampleType",
    "Glue_RndrDvcMngr_IsUpscalerSupported",
    "Glue_RndrDvcMngr_SupportsAsyncCompute",
    "Glue_Resources_GetMaterial",
    "Glue_Resources_GetTexture",
    "Glue_Resources_IsTextureWebStreamingActive",
    "Glue_Resources_HasTextureWebStreamingRequests",
    "Glue_Resources_PrefetchTextureWebStreaming",
    "Glue_Resources_CancelTextureWebStreamingPrefetch",
    "Glue_Resources_SetTextureWebStreamingState",
    "Glue_Resources_GetModel",
    "Glue_Resources_GetModelResourceName",
    "Glue_Resources_GetPhysics",
    "Glue_Resources_GetAnimationGraph",
    "Glue_Resources_GetShader",
    "IAnimParameter_GetName",
    "IAnimParameter_GetParameterType",
    "IAnimParameter_GetDefaultValue",
    "IAnimParameter_GetMinValue",
    "IAnimParameter_GetMaxValue",
    "IAnimParameter_GetNumOptionNames",
    "IAnimParameter_GetOptionName",
    "nmPrmtrnstnc_GetName",
    "nmPrmtrnstnc_GetValue",
    "nmPrmtrnstnc_GetParameterType",
    "nmPrmtrnstnc_IsAutoReset",
    "nmPrmtrnstnc_SetValue",
    "nmPrmtrLst_Count",
    "nmPrmtrLst_GetParameter",
    "nmPrmtrLst_GetParameter_1",
    "IAnimationGraph_DestroyStrongHandle",
    "IAnimationGraph_IsStrongHandleValid",
    "IAnimationGraph_IsError",
    "IAnimationGraph_IsStrongHandleLoaded",
    "IAnimationGraph_CopyStrongHandle",
    "IAnimationGraph_GetBindingPtr",
    "IAnimationGraph_GetGuid",
    "IAnimationGraph_GetResourceName",
    "IAnimationGraph_GetParameterList",
    "syncRsrcDtRqst_GetFileName",
    "syncRsrcDtRqst_GetResultBuffer",
    "syncRsrcDtRqst_GetResultBufferSize",
    "IMaterial2_DestroyStrongHandle",
    "IMaterial2_IsStrongHandleValid",
    "IMaterial2_IsError",
    "IMaterial2_IsStrongHandleLoaded",
    "IMaterial2_CopyStrongHandle",
    "IMaterial2_GetBindingPtr",
    "IMaterial2_GetGuid",
    "IMaterial2_GetName",
    "IMaterial2_GetNameWithMod",
    "IMaterial2_GetSimilarityKey",
    "IMaterial2_IsLoaded",
    "IMaterial2_GetMode",
    "IMaterial2_GetMode_1",
    "IMaterial2_GetMode_2",
    "IMaterial2_GetRenderAttributes",
    "IMaterial2_RecreateAllStaticConstantAndCommandBuffers",
    "IMaterial2_GetFirstTextureAttribute",
    "IMaterial2_GetBoolAttributeOrDefault",
    "IMaterial2_GetIntAttributeOrDefault",
    "IMaterial2_GetFloatAttributeOrDefault",
    "IMaterial2_GetTextureAttributeOrDefault",
    "IMaterial2_HasParam",
    "IMaterial2_Set",
    "IMaterial2_GetString",
    "IMaterial2_Set_1",
    "IMaterial2_GetVector4",
    "IMaterial2_Set_2",
    "IMaterial2_GetTexture",
    "IMaterial2_SetEdited",
    "IMaterial2_IsEdited",
    "IMaterial2_ReloadStaticCombos",
    "IMaterial2_GetPhysicsSurfaceProperties",
    "IPVS_IsEmptyPVS",
    "IPVS_IsInPVS",
    "IPVS_IsAbsBoxInPVS",
    "IPVS_IsSkyVisibleFromPosition",
    "IPVS_IsSunVisibleFromPosition",
    "Physggrgtnstnc_WakeUp",
    "Physggrgtnstnc_PutToSleep",
    "Physggrgtnstnc_IsAsleep",
    "Physggrgtnstnc_SetVelocity",
    "Physggrgtnstnc_AddVelocity",
    "Physggrgtnstnc_SetAngularVelocity",
    "Physggrgtnstnc_AddAngularVelocity",
    "Physggrgtnstnc_GetBodyCount",
    "Physggrgtnstnc_GetBodyHandle",
    "Physggrgtnstnc_GetBodyName",
    "Physggrgtnstnc_GetBodyNameHash",
    "Physggrgtnstnc_GetBodyByNameHash",
    "Physggrgtnstnc_GetBodyIndex",
    "Physggrgtnstnc_FindBodyByName",
    "Physggrgtnstnc_GetJointCount",
    "Physggrgtnstnc_GetJointHandle",
    "Physggrgtnstnc_RemoveJoint",
    "Physggrgtnstnc_GetOrigin",
    "Physggrgtnstnc_GetMassCenter",
    "Physggrgtnstnc_SetSurfaceProperties",
    "Physggrgtnstnc_GetTotalMass",
    "Physggrgtnstnc_SetTotalMass",
    "Physggrgtnstnc_SetLinearDamping",
    "Physggrgtnstnc_SetAngularDamping",
    "Physggrgtnstnc_GetWorld",
    "PhysSrfcPrprtyCn_GetSurfacePropCount",
    "PhysSrfcPrprtyCn_GetSurfaceProperty",
    "PhysSrfcPrprtyCn_AddProperty",
    "IPhysicsBody_GetWorld",
    "IPhysicsBody_SetGravityScale",
    "IPhysicsBody_GetGravityScale",
    "IPhysicsBody_IsGravityEnabled",
    "IPhysicsBody_EnableGravity",
    "IPhysicsBody_SetMass",
    "IPhysicsBody_GetMass",
    "IPhysicsBody_GetMassCenter",
    "IPhysicsBody_GetLocalMassCenter",
    "IPhysicsBody_SetLocalMassCenter",
    "IPhysicsBody_SetOverrideMassCenter",
    "IPhysicsBody_GetOverrideMassCenter",
    "IPhysicsBody_SetPosition",
    "IPhysicsBody_GetPosition",
    "IPhysicsBody_SetOrientation",
    "IPhysicsBody_GetOrientation",
    "IPhysicsBody_SetTransform",
    "IPhysicsBody_GetTransform",
    "IPhysicsBody_SetLinearVelocity",
    "IPhysicsBody_GetLinearVelocity",
    "IPhysicsBody_GetVelocityAtPoint",
    "IPhysicsBody_AddLinearVelocity",
    "IPhysicsBody_SetAngularVelocity",
    "IPhysicsBody_GetAngularVelocity",
    "IPhysicsBody_Wake",
    "IPhysicsBody_Sleep",
    "IPhysicsBody_IsSleeping",
    "IPhysicsBody_EnableAutoSleeping",
    "IPhysicsBody_DisableAutoSleeping",
    "IPhysicsBody_SetSleepThreshold",
    "IPhysicsBody_GetSleepThreshold",
    "IPhysicsBody_EnableTouchEvents",
    "IPhysicsBody_DisableTouchEvents",
    "IPhysicsBody_IsTouchEventEnabled",
    "IPhysicsBody_GetType",
    "IPhysicsBody_SetType",
    "IPhysicsBody_GetShapeCount",
    "IPhysicsBody_GetShape",
    "IPhysicsBody_AddSphereShape",
    "IPhysicsBody_AddCapsuleShape",
    "IPhysicsBody_AddBoxShape",
    "IPhysicsBody_AddHullShape",
    "IPhysicsBody_AddHullShape_1",
    "IPhysicsBody_AddHullShape_2",
    "IPhysicsBody_AddMeshShape",
    "IPhysicsBody_AddMeshShape_1",
    "IPhysicsBody_AddHeightFieldShape",
    "IPhysicsBody_RemoveShape",
    "IPhysicsBody_PurgeShapes",
    "IPhysicsBody_ApplyLinearImpulse",
    "IPhysicsBody_ApplyLinearImpulseAtWorldSpace",
    "IPhysicsBody_ApplyAngularImpulse",
    "IPhysicsBody_ApplyForce",
    "IPhysicsBody_ApplyForceAt",
    "IPhysicsBody_ApplyTorque",
    "IPhysicsBody_ClearForces",
    "IPhysicsBody_ClearTorque",
    "IPhysicsBody_Enable",
    "IPhysicsBody_Disable",
    "IPhysicsBody_IsEnabled",
    "IPhysicsBody_BuildMass",
    "IPhysicsBody_SetMassUpdatesDeferred",
    "IPhysicsBody_SetLinearDamping",
    "IPhysicsBody_GetLinearDamping",
    "IPhysicsBody_SetAngularDamping",
    "IPhysicsBody_GetAngularDamping",
    "IPhysicsBody_BuildBounds",
    "IPhysicsBody_GetDensity",
    "IPhysicsBody_GetClosestPoint",
    "IPhysicsBody_SetMaterialIndex",
    "IPhysicsBody_GetAggregate",
    "IPhysicsBody_SetTargetTransform",
    "IPhysicsBody_CheckOverlap",
    "IPhysicsBody_ComputePenetration",
    "IPhysicsBody_GetScrapeContacts",
    "IPhysicsBody_GetLocalInertiaVector",
    "IPhysicsBody_GetLocalInertiaOrientation",
    "IPhysicsBody_SetLocalInertia",
    "IPhysicsBody_ResetLocalInertia",
    "IPhysicsBody_ManagedObject",
    "IPhysicsBody_SetMotionLocks",
    "IPhysicsBody_IsTouching",
    "IPhysicsBody_IsTouching_1",
    "IPhysicsBody_ResetProxy",
    "IPhysicsBody_SetBullet",
    "IPhysicsBody_ApplyBuoyancyImpulse",
    "IPhysicsBody_ScaleInertia",
    "IPhysicsBody_SetContactRecycling",
    "IPhysicsJoint_GetWorld",
    "IPhysicsJoint_GetBody1",
    "IPhysicsJoint_GetBody2",
    "IPhysicsJoint_GetLocalFrameA",
    "IPhysicsJoint_GetLocalFrameB",
    "IPhysicsJoint_SetLocalFrameA",
    "IPhysicsJoint_SetLocalFrameB",
    "IPhysicsJoint_SetEnableCollision",
    "IPhysicsJoint_IsCollisionEnabled",
    "IPhysicsJoint_GetType",
    "IPhysicsJoint_SetLinearSpring",
    "IPhysicsJoint_GetLinearSpring",
    "IPhysicsJoint_SetAngularSpring",
    "IPhysicsJoint_GetAngularSpring",
    "IPhysicsJoint_SetAngularMotor",
    "IPhysicsJoint_SetLinearMotor",
    "IPhysicsJoint_SetMinLength",
    "IPhysicsJoint_GetMinLength",
    "IPhysicsJoint_SetMaxLength",
    "IPhysicsJoint_GetMaxLength",
    "IPhysicsJoint_SetMinForce",
    "IPhysicsJoint_GetMinForce",
    "IPhysicsJoint_SetMaxForce",
    "IPhysicsJoint_GetMaxForce",
    "IPhysicsJoint_SetFriction",
    "IPhysicsJoint_SetSwingLimit",
    "IPhysicsJoint_SetSwingLimitEnabled",
    "IPhysicsJoint_SetTwistLimits",
    "IPhysicsJoint_SetTwistLimitEnabled",
    "IPhysicsJoint_SetLinearLimits",
    "IPhysicsJoint_SetLinearLimitEnabled",
    "IPhysicsJoint_GetAngle",
    "IPhysicsJoint_GetLinearImpulse",
    "IPhysicsJoint_GetAngularImpulse",
    "IPhysicsJoint_GetMaxLinearImpulse",
    "IPhysicsJoint_GetMaxAngularImpulse",
    "IPhysicsJoint_SetMaxLinearImpulse",
    "IPhysicsJoint_SetMaxAngularImpulse",
    "IPhysicsJoint_SetMotorVelocity",
    "IPhysicsJoint_SetTargetRotation",
    "IPhysicsJoint_Motor_SetLinearVelocity",
    "IPhysicsJoint_Motor_SetAngularVelocity",
    "IPhysicsJoint_Motor_SetMaxVelocityForce",
    "IPhysicsJoint_Motor_SetMaxVelocityTorque",
    "IPhysicsJoint_Motor_SetLinearHertz",
    "IPhysicsJoint_Motor_SetLinearDampingRatio",
    "IPhysicsJoint_Motor_SetAngularHertz",
    "IPhysicsJoint_Motor_SetAngularDampingRatio",
    "IPhysicsJoint_Motor_SetMaxSpringForce",
    "IPhysicsJoint_Motor_SetMaxSpringTorque",
    "IPhysicsJoint_Motor_GetLinearVelocity",
    "IPhysicsJoint_Motor_GetAngularVelocity",
    "IPhysicsJoint_Motor_GetMaxVelocityForce",
    "IPhysicsJoint_Motor_GetMaxVelocityTorque",
    "IPhysicsJoint_Motor_GetLinearHertz",
    "IPhysicsJoint_Motor_GetLinearDampingRatio",
    "IPhysicsJoint_Motor_GetAngularHertz",
    "IPhysicsJoint_Motor_GetAngularDampingRatio",
    "IPhysicsJoint_Motor_GetMaxSpringForce",
    "IPhysicsJoint_Motor_GetMaxSpringTorque",
    "IPhysicsJoint_Wheel_EnableSuspension",
    "IPhysicsJoint_Wheel_IsSuspensionEnabled",
    "IPhysicsJoint_Wheel_SetSuspensionHertz",
    "IPhysicsJoint_Wheel_GetSuspensionHertz",
    "IPhysicsJoint_Wheel_SetSuspensionDampingRatio",
    "IPhysicsJoint_Wheel_GetSuspensionDampingRatio",
    "IPhysicsJoint_Wheel_EnableSuspensionLimit",
    "IPhysicsJoint_Wheel_IsSuspensionLimitEnabled",
    "IPhysicsJoint_Wheel_GetLowerSuspensionLimit",
    "IPhysicsJoint_Wheel_GetUpperSuspensionLimit",
    "IPhysicsJoint_Wheel_SetSuspensionLimits",
    "IPhysicsJoint_Wheel_EnableSpinMotor",
    "IPhysicsJoint_Wheel_IsSpinMotorEnabled",
    "IPhysicsJoint_Wheel_SetSpinMotorSpeed",
    "IPhysicsJoint_Wheel_GetSpinMotorSpeed",
    "IPhysicsJoint_Wheel_SetMaxSpinTorque",
    "IPhysicsJoint_Wheel_GetMaxSpinTorque",
    "IPhysicsJoint_Wheel_EnableSteering",
    "IPhysicsJoint_Wheel_IsSteeringEnabled",
    "IPhysicsJoint_Wheel_SetSteeringHertz",
    "IPhysicsJoint_Wheel_GetSteeringHertz",
    "IPhysicsJoint_Wheel_SetSteeringDampingRatio",
    "IPhysicsJoint_Wheel_GetSteeringDampingRatio",
    "IPhysicsJoint_Wheel_SetMaxSteeringTorque",
    "IPhysicsJoint_Wheel_GetMaxSteeringTorque",
    "IPhysicsJoint_Wheel_EnableSteeringLimit",
    "IPhysicsJoint_Wheel_IsSteeringLimitEnabled",
    "IPhysicsJoint_Wheel_GetLowerSteeringLimit",
    "IPhysicsJoint_Wheel_GetUpperSteeringLimit",
    "IPhysicsJoint_Wheel_SetSteeringLimits",
    "IPhysicsJoint_Wheel_SetTargetSteeringAngle",
    "IPhysicsJoint_Wheel_GetTargetSteeringAngle",
    "IPhysicsJoint_Wheel_GetSpinSpeed",
    "IPhysicsJoint_Wheel_GetSpinTorque",
    "IPhysicsJoint_Wheel_GetSteeringAngle",
    "IPhysicsJoint_Wheel_GetSteeringTorque",
    "IPhysicsJoint_Parallel_SetHertz",
    "IPhysicsJoint_Parallel_GetHertz",
    "IPhysicsJoint_Parallel_SetDampingRatio",
    "IPhysicsJoint_Parallel_GetDampingRatio",
    "IPhysicsJoint_Parallel_SetMaxTorque",
    "IPhysicsJoint_Parallel_GetMaxTorque",
    "IPhysicsShape_AddCollisionFunctionMask",
    "IPhysicsShape_RemoveCollisionFunctionMask",
    "IPhysicsShape_GetCollisionFunctionMask",
    "IPhysicsShape_HasTag",
    "IPhysicsShape_AddTag",
    "IPhysicsShape_RemoveTag",
    "IPhysicsShape_ClearTags",
    "IPhysicsShape_GetBody",
    "IPhysicsShape_SetMaterialIndex",
    "IPhysicsShape_SetSurfaceIndex",
    "IPhysicsShape_GetMaterialName",
    "IPhysicsShape_GetType",
    "IPhysicsShape_UpdateMeshShape",
    "IPhysicsShape_UpdateHeightShape",
    "IPhysicsShape_UpdateSphereShape",
    "IPhysicsShape_UpdateCapsuleShape",
    "IPhysicsShape_UpdateHullShape",
    "IPhysicsShape_ManagedObject",
    "IPhysicsShape_SetTrigger",
    "IPhysicsShape_IsTrigger",
    "IPhysicsShape_GetTriangulation",
    "IPhysicsShape_GetTriangulationForNavmesh",
    "IPhysicsShape_GetOutline",
    "IPhysicsShape_AsSphere",
    "IPhysicsShape_AsCapsule",
    "IPhysicsShape_UpdateBoxShape",
    "IPhysicsShape_SetFriction",
    "IPhysicsShape_GetFriction",
    "IPhysicsShape_SetLocalVelocity",
    "IPhysicsShape_GetLocalVelocity",
    "IPhysicsShape_SetElasticity",
    "IPhysicsShape_SetRollingResistance",
    "IPhysicsShape_SetIgnoreTraces",
    "IPhysicsShape_SetHasNoMass",
    "IPhysicsShape_BuildBounds",
    "IPhysicsShape_LocalBounds",
    "IPhysicsShape_IsTouching",
    "IPhysicsShape_ResetProxy",
    "IPhysicsWorld_AddBody",
    "IPhysicsWorld_AddBody_1",
    "IPhysicsWorld_RemoveBody",
    "IPhysicsWorld_GetWorldReferenceBody",
    "IPhysicsWorld_SetWorldReferenceBody",
    "IPhysicsWorld_RemoveJoint",
    "IPhysicsWorld_SetGravity",
    "IPhysicsWorld_GetGravity",
    "IPhysicsWorld_SetSimulation",
    "IPhysicsWorld_GetSimulation",
    "IPhysicsWorld_EnableSleeping",
    "IPhysicsWorld_DisableSleeping",
    "IPhysicsWorld_IsSleepingEnabled",
    "IPhysicsWorld_SetMaximumLinearSpeed",
    "IPhysicsWorld_AddWeldJoint",
    "IPhysicsWorld_AddSpringJoint",
    "IPhysicsWorld_AddRevoluteJoint",
    "IPhysicsWorld_AddPrismaticJoint",
    "IPhysicsWorld_AddSphericalJoint",
    "IPhysicsWorld_AddMotorJoint",
    "IPhysicsWorld_AddWheelJoint",
    "IPhysicsWorld_AddParallelJoint",
    "IPhysicsWorld_AddFilterJoint",
    "IPhysicsWorld_SetCollisionRulesFromJson",
    "IPhysicsWorld_StepSimulation",
    "IPhysicsWorld_ProcessIntersections",
    "IPhysicsWorld_DestroyAggregateInstance",
    "IPhysicsWorld_CreateAggregateInstance",
    "IPhysicsWorld_CreateAggregateInstance_1",
    "IPhysicsWorld_SetDebugScene",
    "IPhysicsWorld_GetDebugScene",
    "IPhysicsWorld_Draw",
    "IPhysicsWorld_ManagedObject",
    "IPhysicsWorld_Query",
    "IPhysicsWorld_Query_1",
    "IPhysicsWorld_Query_2",
    "RD_RgstrRsrcDttl_SetDataRegistrationFailed",
    "RD_RgstrRsrcDttl_IsReloading",
    "RD_RgstrRsrcDttl_SetFinalResourceData",
    "RD_RgstrRsrcDttl_GetDataRegistrationFailed",
    "RD_RgstrRsrcDttl_GetFinalResourceData",
    "RD_RgstrRsrcDttl_GetResultBufferSize",
    "RyTrcScnWrld_BeginBuild",
    "RyTrcScnWrld_AddSceneWorldToBuild",
    "RyTrcScnWrld_EndBuild",
    "RyTrcScnWrld_BuildTLASForWorld",
    "IRenderContext_Draw",
    "IRenderContext_DrawInstanced",
    "IRenderContext_DrawIndexed",
    "IRenderContext_DrawIndexedInstanced",
    "IRenderContext_DrawInstancedIndirect",
    "IRenderContext_DrawIndexedInstancedIndirect",
    "IRenderContext_SemaphoreSignalAtEnd",
    "IRenderContext_SemaphoreWaitAtBegin",
    "IRenderContext_TextureBarrierTransition",
    "IRenderContext_BufferBarrierTransition",
    "IRenderContext_SetScissorRect",
    "IRenderContext_GetAttributesPtrForModify",
    "IRenderContext_GenerateMipMaps",
    "IRenderContext_Clear",
    "IRenderContext_ClearTexture",
    "IRenderContext_ClearTextureUint",
    "IRenderContext_BindRenderTargets",
    "IRenderContext_BindRenderTargets_1",
    "IRenderContext_RestoreRenderTargets",
    "IRenderContext_SetViewport",
    "IRenderContext_SetViewport_1",
    "IRenderContext_SetViewport_2",
    "IRenderContext_GetViewport",
    "IRenderContext_Submit",
    "IRenderContext_ForceFlushGPU",
    "IRenderContext_BindRenderTargets_2",
    "IRenderContext_BindIndexBuffer",
    "IRenderContext_BindIndexBuffer_1",
    "IRenderContext_BindVertexBuffer",
    "IRenderContext_BindVertexBuffer_1",
    "IRenderContext_BindVertexBuffer_2",
    "IRenderContext_BindVertexBuffer_3",
    "IRenderContext_BindVertexShader",
    "IRenderContext_BindPixelShader",
    "IRenderContext_SetDynamicConstantBufferData",
    "IRenderContext_BindTexture",
    "IRenderContext_ReadTexturePixels",
    "IRenderContext_ReadBuffer",
    "IRenderContext_BeginPixEvent",
    "IRenderContext_EndPixEvent",
    "IRenderContext_PixSetMarker",
    "ISceneLayer_SetObjectMatchID",
    "ISceneLayer_AddObjectFlagsRequiredMask",
    "ISceneLayer_AddObjectFlagsExcludedMask",
    "ISceneLayer_RemoveObjectFlagsRequiredMask",
    "ISceneLayer_RemoveObjectFlagsExcludedMask",
    "ISceneLayer_GetObjectFlagsRequiredMask",
    "ISceneLayer_GetObjectFlagsExcludedMask",
    "ISceneLayer_GetDebugName",
    "ISceneLayer_AddSemaphoreDependency",
    "ISceneLayer_SetSemaphore",
    "ISceneLayer_GetRenderAttributesPtr",
    "ISceneLayer_SetAttr",
    "ISceneLayer_SetLayerNoCull",
    "ISceneLayer_SetBoundingVolumeSizeCullThresholdInPercent",
    "ISceneLayer_DisableReflectionsAndRefractionsForObjectsInThisLayer",
    "ISceneLayer_SetFadeTargetLayer",
    "ISceneLayer_SetTonemapOverrideScaleValue",
    "ISceneLayer_SetShaderMode",
    "ISceneLayer_SetWireframeOverride",
    "ISceneLayer_SetClearColor",
    "ISceneLayer_GetTextureValue",
    "ISceneLayer_GetTextureValue_1",
    "ISceneLayer_GetColorTarget",
    "ISceneLayer_GetDepthTarget",
    "ISceneLayer_SetOutput",
    "ISceneLayer_SetOutput_1",
    "ISceneLayer_SetOverlayStencil",
    "Get__ISceneLayer_m_nLayerFlags",
    "Set__ISceneLayer_m_nLayerFlags",
    "Get__ISceneLayer_LayerEnum",
    "Set__ISceneLayer_LayerEnum",
    "Get__ISceneLayer_m_viewport",
    "Set__ISceneLayer_m_viewport",
    "Get__ISceneLayer_m_nClearFlags",
    "Set__ISceneLayer_m_nClearFlags",
    "ISceneView_GetMainViewport",
    "ISceneView_GetSwapChain",
    "ISceneView_GetRenderTargetArrayIndex",
    "ISceneView_AddDependentView",
    "ISceneView_GetRenderAttributesPtr",
    "ISceneView_AddRenderLayer",
    "ISceneView_AddLabelLayer",
    "ISceneView_AddManagedProceduralLayer",
    "ISceneView_SetOcclusionCull",
    "ISceneView_GetOcclusionCullRequested",
    "ISceneView_AddBlitTextureToTargetLayer",
    "ISceneView_SetDefaultLayerObjectRequiredFlags",
    "ISceneView_SetDefaultLayerObjectExcludedFlags",
    "ISceneView_GetDefaultLayerObjectRequiredFlags",
    "ISceneView_GetDefaultLayerObjectExcludedFlags",
    "ISceneView_AddWorldToRenderList",
    "ISceneView_GetCameraPosition",
    "ISceneView_FindOrCreateRenderTarget",
    "ISceneView_SetParent",
    "ISceneView_GetParent",
    "ISceneView_GetPriority",
    "ISceneView_SetPriority",
    "ISceneView_GetFrustum",
    "ISceneView_GetPostProcessEnabled",
    "ISceneView_GetToolsVisMode",
    "Get__ISceneView_m_ViewUniqueId",
    "Set__ISceneView_m_ViewUniqueId",
    "Get__ISceneView_m_ManagedCameraId",
    "Set__ISceneView_m_ManagedCameraId",
    "ISceneWorld_DeleteAllObjects",
    "ISceneWorld_Release",
    "ISceneWorld_GetSceneObjectCount",
    "ISceneWorld_IsEmpty",
    "ISceneWorld_GetWorldDebugName",
    "ISceneWorld_SetDeleteAtEndOfFrame",
    "ISceneWorld_GetDeleteAtEndOfFrame",
    "ISceneWorld_DeleteEndOfFrameObjects",
    "ISceneWorld_MeshTrace",
    "ISceneWorld_MeshTraceAll",
    "ISceneWorld_GetPVS",
    "ISceneWorld_SetPVS",
    "ISceneWorld_Add3DSkyboxWorld",
    "ISceneWorld_Remove3DSkyboxWorld",
    "ISceneWorld_Set3DSkyboxParameters",
    "ISceneWorld_Get3DSkyboxOriginAndScale",
    "ShdrCmplCntxt_Delete",
    "ShdrCmplCntxt_SetMaskedCode",
    "ISteamApps_BIsAppInstalled",
    "ISteamApps_BIsCybercafe",
    "ISteamApps_BIsDlcInstalled",
    "ISteamApps_BIsLowViolence",
    "ISteamApps_BIsSubscribed",
    "ISteamApps_BIsSubscribedApp",
    "ISteamApps_GetAvailableGameLanguages",
    "ISteamApps_GetCurrentGameLanguage",
    "ISteamApps_GetAppBuildId",
    "ISteamApps_BIsVACBanned",
    "ISteamApps_GetCommandLine",
    "ISteamApps_GetAppInstallDir",
    "ISteamFriends_GetProfileItemPropertyString",
    "ISteamFriends_RequestEquippedProfileItems",
    "ISteamFriends_GetPersonaName",
    "ISteamFriends_SetRichPresence",
    "ISteamFriends_ClearRichPresence",
    "ISteamGameServer_SetServerName",
    "ISteamGameServer_SetMapName",
    "ISteamGameServer_SetGameTags",
    "ISteamGameServer_SetDedicatedServer",
    "ISteamGameServer_SetAdvertiseServerActive",
    "ISteamGameServer_SetMaxPlayerCount",
    "ISteamGameServer_SetPasswordProtected",
    "ISteamGameServer_LogOnAnonymous",
    "ISteamGameServer_LogOn",
    "ISteamGameServer_LogOff",
    "ISteamGameServer_SetGameDescription",
    "ISteamGameServer_SetProduct",
    "ISteamGameServer_SetModDir",
    "ISteamGameServer_BLoggedOn",
    "ISteamGameServer_GetPublicIP",
    "ISteamGameServer_GetAuthSessionTicket",
    "ISteamGameServer_BeginAuthSession",
    "ISteamGameServer_CancelAuthTicket",
    "ISteamGameServer_EndAuthSession",
    "StmHTMLSrfc_Init",
    "StmHTMLSrfc_Shutdown",
    "StmHTMLSrfc_CreateBrowser",
    "StmHTMLSrfc_RemoveBrowser",
    "StmHTMLSrfc_LoadURL",
    "StmHTMLSrfc_AddHeader",
    "StmHTMLSrfc_ExecuteJavascript",
    "StmHTMLSrfc_SetSize",
    "StmHTMLSrfc_GetLinkAtPosition",
    "StmHTMLSrfc_SetHorizontalScroll",
    "StmHTMLSrfc_SetVerticalScroll",
    "StmHTMLSrfc_SetKeyFocus",
    "StmHTMLSrfc_AllowStartRequest",
    "StmHTMLSrfc_JSDialogResponse",
    "StmHTMLSrfc_SetBackgroundMode",
    "StmHTMLSrfc_SetDPIScalingFactor",
    "StmHTMLSrfc_KeyDown",
    "StmHTMLSrfc_KeyUp",
    "StmHTMLSrfc_KeyChar",
    "StmHTMLSrfc_MouseUp",
    "StmHTMLSrfc_MouseDown",
    "StmHTMLSrfc_MouseDoubleClick",
    "StmHTMLSrfc_MouseMove",
    "StmHTMLSrfc_MouseWheel",
    "StmHTMLSrfc_SetCookie",
    "StmMtchmkng_LeaveLobby",
    "StmMtchmkng_GetNumLobbyMembers",
    "StmMtchmkng_GetLobbyMemberByIndex",
    "StmMtchmkng_GetLobbyData",
    "StmMtchmkng_SetLobbyData",
    "StmMtchmkng_GetLobbyDataCount",
    "StmMtchmkng_DeleteLobbyData",
    "StmMtchmkng_GetLobbyDataByIndex",
    "StmNtwrkngMssgs_GetSessionState",
    "StmNtwrkngMssgs_SendMessageToUser",
    "StmNtwrkngMssgs_ReceiveMessagesOnChannel",
    "StmNtwrkngMssgs_AcceptSessionWithUser",
    "StmNtwrkngMssgs_CloseChannelWithUser",
    "StmNtwrkngMssgs_ReleaseMessage",
    "StmNtwrkngMssgs_GetConnectionInfo",
    "StmNtwrkngSckts_GetConnectionInfo",
    "StmNtwrkngSckts_StartAuthentication",
    "StmNtwrkngSckts_BeginRequestFakeIP",
    "StmNtwrkngSckts_ReleaseMessage",
    "StmNtwrkngtls_SetConfig",
    "StmNtwrkngtls_SetConfig_1",
    "StmNtwrkngtls_InitializeRelayNetwork",
    "StmNtwrkngtls_GetLocalPingLocationString",
    "StmNtwrkngtls_EstimatePingFromLocationString",
    "StmRmtStrg_FileExists",
    "StmRmtStrg_GetFileSize",
    "StmRmtStrg_FileRead",
    "StmRmtStrg_FileWrite",
    "StmRmtStrg_FileWriteAsync",
    "StmRmtStrg_FileDelete",
    "ISteamUser_BLoggedOn",
    "ISteamUser_GetSteamID",
    "ISteamUser_GetAuthSessionTicket",
    "ISteamUser_BeginAuthSession",
    "ISteamUser_CancelAuthTicket",
    "ISteamUser_EndAuthSession",
    "ISteamUtils_InitFilterText",
    "ISteamUtils_FilterText",
    "ITonemapSystem_SetTonemapParameters",
    "ITonemapSystem_ResetTonemapParameters",
    "IVfx_Init",
    "IVfx_CompileShader",
    "IVfx_ClearShaderCache",
    "IVfx_CreateSharedContext",
    "IVolumetricFog_IsFoggingEnabled",
    "IVolumetricFog_SetParams",
    "IWorldReference_Release",
    "IWorldReference_IsWorldLoaded",
    "IWorldReference_IsErrorWorld",
    "IWorldReference_IsMarkedForDeletion",
    "IWorldReference_GetWorldBounds",
    "IWorldReference_GetSceneWorld",
    "IWorldReference_PrecacheAllWorldNodes",
    "IWorldReference_GetFolder",
    "IWorldReference_GetEntityCount",
    "IWorldReference_GetEntityKeyValues",
    "IWorldReference_SetWorldTransform",
    "ImageLoader_GetMemRequired",
    "ImageLoader_GetMemRequired_1",
    "ImageLoader_ConvertImageFormat",
    "KeyValues3_DeleteThis",
    "KeyValues3_Create",
    "KeyValues3_IsArray",
    "KeyValues3_IsTable",
    "KeyValues3_GetType",
    "KeyValues3_GetValueBool",
    "KeyValues3_GetValueInt",
    "KeyValues3_GetValueInt64",
    "KeyValues3_GetValueUint64",
    "KeyValues3_GetValueFloat",
    "KeyValues3_GetValueDouble",
    "KeyValues3_GetValueString",
    "KeyValues3_GetValueVector",
    "KeyValues3_GetValueColor",
    "KeyValues3_SetValueBool",
    "KeyValues3_SetValueString",
    "KeyValues3_SetValueResourceString",
    "KeyValues3_SetValueInt",
    "KeyValues3_SetValueFloat",
    "KeyValues3_SetMemberString",
    "KeyValues3_SetMemberInt",
    "KeyValues3_SetMemberFloat",
    "KeyValues3_GetMemberString",
    "KeyValues3_GetMemberInt",
    "KeyValues3_GetMemberFloat",
    "KeyValues3_GetMemberVector",
    "KeyValues3_SetToEmptyArray",
    "KeyValues3_GetArrayLength",
    "KeyValues3_ArrayAddToTail",
    "KeyValues3_GetArrayElement",
    "KeyValues3_FindOrCreateMember",
    "KeyValues3_SetToEmptyTable",
    "KeyValues3_GetMemberCount",
    "KeyValues3_GetMember",
    "KeyValues3_GetMemberName",
    "KeyValues3_Save",
    "LZ4Glue_CompressBound",
    "LZ4Glue_Compress",
    "LZ4Glue_CompressHC",
    "LZ4Glue_Decompress",
    "LZ4Glue_CompressFrameBound",
    "LZ4Glue_CompressFrame",
    "LZ4Glue_GetFrameContentSize",
    "LZ4Glue_DecompressFrame",
    "MeshGlue_CreateRenderMesh",
    "MeshGlue_GetModelNumVertices",
    "MeshGlue_GetModelVertices",
    "MeshGlue_GetModelNumIndices",
    "MeshGlue_GetModelIndices",
    "MeshGlue_GetDrawCallVertexCount",
    "MeshGlue_GetDrawCallIndexCount",
    "MeshGlue_GetDrawCallStreams",
    "MeshGlue_ReadDrawCall",
    "MeshGlue_GetModelIndexCount",
    "MeshGlue_GetModelIndexStart",
    "MeshGlue_GetModelBaseVertex",
    "MeshGlue_GetModelLodIndexCount",
    "MeshGlue_SetMeshMaterial",
    "MeshGlue_SetMeshPrimType",
    "MeshGlue_SetMeshBounds",
    "MeshGlue_SetMeshUvDensity",
    "MeshGlue_SetMeshVertexRange",
    "MeshGlue_SetMeshIndexRange",
    "MeshGlue_AddMeshDrawCall",
    "MeshGlue_SetMeshVertexBuffer",
    "MeshGlue_SetMeshIndexBuffer",
    "MeshGlue_CreateVertexBuffer",
    "MeshGlue_CreateVertexBuffer_1",
    "MeshGlue_CreateIndexBuffer",
    "MeshGlue_LockVertexBuffer",
    "MeshGlue_UnlockVertexBuffer",
    "MeshGlue_LockIndexBuffer",
    "MeshGlue_UnlockIndexBuffer",
    "MeshGlue_SetVertexBufferData",
    "MeshGlue_SetIndexBufferData",
    "MeshGlue_SetVertexBufferSize",
    "MeshGlue_SetIndexBufferSize",
    "MeshGlue_TriangulatePolygon",
    "MeshGlue_ClipPolygonLineSegment",
    "MeshGlue_GenerateUVsForTriangles",
    "MeshGlue_SerializeMeshBuffers",
    "MeshGlue_SerializeMeshData",
    "MeshGlue_SerializeModelData",
    "MeshGlue_SerializePhysicsData",
    "MeshGlue_SerializePhysicsDataFromAggregate",
    "MeshGlue_BuildAggregateData",
    "MeshGlue_SetMeshMorphData",
    "NativeLowLevel_Copy",
    "NativeLowLevel_MurmurHash64",
    "PerformanceTrace_BeginEvent",
    "PerformanceTrace_EndEvent",
    "PhysicsTrace_Trace",
    "PhysicsTrace_TraceAll",
    "PhysicsTrace_TraceAgainstCapsule",
    "PhysicsTrace_TraceAgainstBBox",
    "PhysicsTrace_TraceAgainstSphere",
    "RndrPplnTls_GetTargetFormat",
    "RndrPplnTls_GetTargetWidth",
    "RndrPplnTls_GetTargetHeight",
    "RndrPplnTls_TargetSupportsSrgbWrite",
    "RndrPplnTls_ScreenText",
    "RndrPplnTls_CollectLoadedShaderNames",
    "RndrPplnTls_GetLoadedShaderName",
    "RndrPplnTls_GetShaderIDColor",
    "RndrPplnTls_AddTextureDebugLayer",
    "RndrPplnTls_FindOrCreateScratchTarget",
    "RndrPplnTls_IsFloatFormat",
    "RndrPplnTls_GetHdrFormat",
    "RndrPplnTls_GetBackBufferFormat",
    "RenderTools_SetRenderState",
    "RenderTools_Draw",
    "RenderTools_ResolveFrameBuffer",
    "RenderTools_ResolveDepthBuffer",
    "RenderTools_DrawSceneObject",
    "RenderTools_DrawModel",
    "RenderTools_BindModelDrawCall",
    "RenderTools_SetRenderStateForModelDrawCall",
    "RenderTools_GetModelMeshSkinning",
    "RenderTools_GetModelMeshSkinningBones",
    "RenderTools_AllocateVertexCache",
    "RenderTools_GetVertexCacheBuffer",
    "RenderTools_DispatchModelMeshSkinning",
    "RenderTools_BindModelDrawCallVertexIds",
    "RenderTools_BindDepthTarget",
    "RenderTools_BindColorAndDepthTarget",
    "RenderTools_BlitTexture",
    "RenderTools_MakeFrameBufferCopy",
    "RenderTools_PackSceneEnvMap",
    "RenderTools_GetModelDrawCallTint",
    "RenderTools_GetSceneObjectLightmapKey",
    "RenderTools_GetSceneWorldWind",
    "RenderTools_DrawDynamicSceneObject",
    "RenderTools_ModelMeshHasMorphs",
    "RenderTools_QueueModelMeshMorph",
    "RenderTools_GenerateMorphs",
    "RenderTools_GetMorphSubrect",
    "RenderTools_CreateConstantBuffer",
    "RenderTools_SetConstantBufferData",
    "RenderTools_DestroyConstantBuffer",
    "RenderTools_GetLightProbeVolume",
    "RenderTools_SetLightProbeVolumeAttributes",
    "RenderTools_DrawSkyBox",
    "RenderTools_GetSceneObjectDraw",
    "RenderTools_TonemapFrameUpdate",
    "RenderTools_RenderAutoExposure",
    "RenderTools_DrawModelDrawCall",
    "RenderTools_DrawModelDrawCallIndirect",
    "RenderTools_GetMaterialBindSize",
    "RenderTools_ResetMaterialBind",
    "RenderTools_GetMaterialSimilarityKey",
    "RenderTools_PackSceneLight",
    "RenderTools_SetSceneObjectLightmapAttributes",
    "RenderTools_DrawModel_1",
    "RenderTools_DrawModel_2",
    "RenderTools_DrawModel_3",
    "RenderTools_DrawModel_4",
    "RenderTools_Compute",
    "RenderTools_ComputeIndirect",
    "RenderTools_TraceRays",
    "RenderTools_TraceRaysIndirect",
    "RenderTools_SetDynamicConstantBufferData",
    "RenderTools_CopyTexture",
    "RenderTools_SetGPUBufferData",
    "RenderTools_FillGPUBuffer",
    "RenderTools_CopyGPUBuffer",
    "RenderTools_CopyGPUBufferHiddenStructureCount",
    "RenderTools_SetGPUBufferHiddenStructureCount",
    "RnCapsuleDesc_t_GetOutline",
    "Get__RnCapsuleDesc_t_m_Capsule",
    "Set__RnCapsuleDesc_t_m_Capsule",
    "Get__RnCapsuleDesc_t_m_nCollisionAttributeIndex",
    "Set__RnCapsuleDesc_t_m_nCollisionAttributeIndex",
    "Get__RnCapsuleDesc_t_m_nSurfacePropertyIndex",
    "Set__RnCapsuleDesc_t_m_nSurfacePropertyIndex",
    "RnHullDesc_t_GetHull",
    "Get__RnHullDesc_t_m_nCollisionAttributeIndex",
    "Set__RnHullDesc_t_m_nCollisionAttributeIndex",
    "Get__RnHullDesc_t_m_nSurfacePropertyIndex",
    "Set__RnHullDesc_t_m_nSurfacePropertyIndex",
    "RnHull_t_GetVertexCount",
    "RnHull_t_GetEdgeCount",
    "RnHull_t_GetVolume",
    "RnHull_t_GetMassCenter",
    "RnHull_t_GetCentroid",
    "RnHull_t_GetSurfaceArea",
    "RnHull_t_GetMemory",
    "RnHull_t_GetBbox",
    "RnHull_t_GetVertex",
    "RnHull_t_GetEdgeVertex",
    "RnMeshDesc_t_GetMesh",
    "Get__RnMeshDesc_t_m_nCollisionAttributeIndex",
    "Set__RnMeshDesc_t_m_nCollisionAttributeIndex",
    "Get__RnMeshDesc_t_m_nSurfacePropertyIndex",
    "Set__RnMeshDesc_t_m_nSurfacePropertyIndex",
    "RnMesh_t_GetTriangleCount",
    "RnMesh_t_GetVertexCount",
    "RnMesh_t_GetVertex",
    "RnMesh_t_GetHeight",
    "RnMesh_t_GetMemory",
    "RnMesh_t_GetBbox",
    "RnMesh_t_GetMaterialCount",
    "RnMesh_t_GetTriangleMaterial",
    "RnMesh_t_GetTriangleIndices",
    "RnMesh_t_GetTriangle",
    "RnSphereDesc_t_GetOutline",
    "Get__RnSphereDesc_t_m_Sphere",
    "Set__RnSphereDesc_t_m_Sphere",
    "Get__RnSphereDesc_t_m_nCollisionAttributeIndex",
    "Set__RnSphereDesc_t_m_nCollisionAttributeIndex",
    "Get__RnSphereDesc_t_m_nSurfacePropertyIndex",
    "Set__RnSphereDesc_t_m_nSurfacePropertyIndex",
    "Get__ScnSystmPrFrmStt_m_nTrianglesRendered",
    "Set__ScnSystmPrFrmStt_m_nTrianglesRendered",
    "Get__ScnSystmPrFrmStt_m_nArtistTrianglesRendered",
    "Set__ScnSystmPrFrmStt_m_nArtistTrianglesRendered",
    "Get__ScnSystmPrFrmStt_m_nRenderBatchDraws",
    "Set__ScnSystmPrFrmStt_m_nRenderBatchDraws",
    "Get__ScnSystmPrFrmStt_m_nDrawCalls",
    "Set__ScnSystmPrFrmStt_m_nDrawCalls",
    "Get__ScnSystmPrFrmStt_m_nDrawPrimitives",
    "Set__ScnSystmPrFrmStt_m_nDrawPrimitives",
    "Get__ScnSystmPrFrmStt_m_nBaseSceneObjectPrimDraws",
    "Set__ScnSystmPrFrmStt_m_nBaseSceneObjectPrimDraws",
    "Get__ScnSystmPrFrmStt_m_nAnimatableObjectPrimDraws",
    "Set__ScnSystmPrFrmStt_m_nAnimatableObjectPrimDraws",
    "Get__ScnSystmPrFrmStt_m_nAggregateSceneObjectPrimDraws",
    "Set__ScnSystmPrFrmStt_m_nAggregateSceneObjectPrimDraws",
    "Get__ScnSystmPrFrmStt_m_nAggregateSceneObjectsFullyCulled",
    "Set__ScnSystmPrFrmStt_m_nAggregateSceneObjectsFullyCulled",
    "Get__ScnSystmPrFrmStt_m_nAggregateSceneObjectDrawCalls",
    "Set__ScnSystmPrFrmStt_m_nAggregateSceneObjectDrawCalls",
    "Get__ScnSystmPrFrmStt_m_nAggregateSceneObjectIndirectSubmits",
    "Set__ScnSystmPrFrmStt_m_nAggregateSceneObjectIndirectSubmits",
    "Get__ScnSystmPrFrmStt_m_nAggregateSceneObjectIndirectFragments",
    "Set__ScnSystmPrFrmStt_m_nAggregateSceneObjectIndirectFragments",
    "Get__ScnSystmPrFrmStt_m_nNumMaterialCompute",
    "Set__ScnSystmPrFrmStt_m_nNumMaterialCompute",
    "Get__ScnSystmPrFrmStt_m_nNumMaterialSet",
    "Set__ScnSystmPrFrmStt_m_nNumMaterialSet",
    "Get__ScnSystmPrFrmStt_m_nNumSimilarMaterialSet",
    "Set__ScnSystmPrFrmStt_m_nNumSimilarMaterialSet",
    "Get__ScnSystmPrFrmStt_m_nNumTextureOnlyMaterialSet",
    "Set__ScnSystmPrFrmStt_m_nNumTextureOnlyMaterialSet",
    "Get__ScnSystmPrFrmStt_m_nNumVfxEval",
    "Set__ScnSystmPrFrmStt_m_nNumVfxEval",
    "Get__ScnSystmPrFrmStt_m_nNumVfxRule",
    "Set__ScnSystmPrFrmStt_m_nNumVfxRule",
    "Get__ScnSystmPrFrmStt_m_nNumConstantBufferUpdates",
    "Set__ScnSystmPrFrmStt_m_nNumConstantBufferUpdates",
    "Get__ScnSystmPrFrmStt_m_nNumConstantBufferBytes",
    "Set__ScnSystmPrFrmStt_m_nNumConstantBufferBytes",
    "Get__ScnSystmPrFrmStt_m_nMaterialChangesNonShadow",
    "Set__ScnSystmPrFrmStt_m_nMaterialChangesNonShadow",
    "Get__ScnSystmPrFrmStt_m_nMaterialChangesNonShadowInitial",
    "Set__ScnSystmPrFrmStt_m_nMaterialChangesNonShadowInitial",
    "Get__ScnSystmPrFrmStt_m_nMaterialChangesShadow",
    "Set__ScnSystmPrFrmStt_m_nMaterialChangesShadow",
    "Get__ScnSystmPrFrmStt_m_nMaterialChangesShadowInitial",
    "Set__ScnSystmPrFrmStt_m_nMaterialChangesShadowInitial",
    "Get__ScnSystmPrFrmStt_m_nMaterialChangesShadowAlphaTested",
    "Set__ScnSystmPrFrmStt_m_nMaterialChangesShadowAlphaTested",
    "Get__ScnSystmPrFrmStt_m_nCopyMaterialChangesNonShadow",
    "Set__ScnSystmPrFrmStt_m_nCopyMaterialChangesNonShadow",
    "Get__ScnSystmPrFrmStt_m_nMaxTransformRow",
    "Set__ScnSystmPrFrmStt_m_nMaxTransformRow",
    "Get__ScnSystmPrFrmStt_m_nNumRowsUsed",
    "Set__ScnSystmPrFrmStt_m_nNumRowsUsed",
    "Get__ScnSystmPrFrmStt_m_nNumObjectsTested",
    "Set__ScnSystmPrFrmStt_m_nNumObjectsTested",
    "Get__ScnSystmPrFrmStt_m_nNumObjectsPreCullCheck",
    "Set__ScnSystmPrFrmStt_m_nNumObjectsPreCullCheck",
    "Get__ScnSystmPrFrmStt_m_nNumObjectsPassingCullCheck",
    "Set__ScnSystmPrFrmStt_m_nNumObjectsPassingCullCheck",
    "Get__ScnSystmPrFrmStt_m_nNumVerticesReferenced",
    "Set__ScnSystmPrFrmStt_m_nNumVerticesReferenced",
    "Get__ScnSystmPrFrmStt_m_nNumPrimaryContexts",
    "Set__ScnSystmPrFrmStt_m_nNumPrimaryContexts",
    "Get__ScnSystmPrFrmStt_m_nNumSecondaryContexts",
    "Set__ScnSystmPrFrmStt_m_nNumSecondaryContexts",
    "Get__ScnSystmPrFrmStt_m_nNumDisplayListsSubmitted",
    "Set__ScnSystmPrFrmStt_m_nNumDisplayListsSubmitted",
    "Get__ScnSystmPrFrmStt_m_nNumViewsRendered",
    "Set__ScnSystmPrFrmStt_m_nNumViewsRendered",
    "Get__ScnSystmPrFrmStt_m_nNumResolves",
    "Set__ScnSystmPrFrmStt_m_nNumResolves",
    "Get__ScnSystmPrFrmStt_m_nNumCullBoxes",
    "Set__ScnSystmPrFrmStt_m_nNumCullBoxes",
    "Get__ScnSystmPrFrmStt_m_nCullingBoxCycleCount",
    "Set__ScnSystmPrFrmStt_m_nCullingBoxCycleCount",
    "Get__ScnSystmPrFrmStt_m_nNumObjectsTestedAgainstCullingBoxes",
    "Set__ScnSystmPrFrmStt_m_nNumObjectsTestedAgainstCullingBoxes",
    "Get__ScnSystmPrFrmStt_m_nNumObjectsRejectedByBoundsIndex",
    "Set__ScnSystmPrFrmStt_m_nNumObjectsRejectedByBoundsIndex",
    "Get__ScnSystmPrFrmStt_m_nNumObjectsRejectedByCullBoxes",
    "Set__ScnSystmPrFrmStt_m_nNumObjectsRejectedByCullBoxes",
    "Get__ScnSystmPrFrmStt_m_nNumObjectsRejectedByVis",
    "Set__ScnSystmPrFrmStt_m_nNumObjectsRejectedByVis",
    "Get__ScnSystmPrFrmStt_m_nNumObjectsRejectedByBackfaceCulling",
    "Set__ScnSystmPrFrmStt_m_nNumObjectsRejectedByBackfaceCulling",
    "Get__ScnSystmPrFrmStt_m_nNumObjectsRejectedByScreenSizeCulling",
    "Set__ScnSystmPrFrmStt_m_nNumObjectsRejectedByScreenSizeCulling",
    "Get__ScnSystmPrFrmStt_m_nNumObjectsRejectedByFading",
    "Set__ScnSystmPrFrmStt_m_nNumObjectsRejectedByFading",
    "Get__ScnSystmPrFrmStt_m_nNumFadingObjects",
    "Set__ScnSystmPrFrmStt_m_nNumFadingObjects",
    "Get__ScnSystmPrFrmStt_m_nNumUniqueMaterialsSeen",
    "Set__ScnSystmPrFrmStt_m_nNumUniqueMaterialsSeen",
    "Get__ScnSystmPrFrmStt_m_nNumUnshadowedLightsInView",
    "Set__ScnSystmPrFrmStt_m_nNumUnshadowedLightsInView",
    "Get__ScnSystmPrFrmStt_m_nNumShadowedLightsInView",
    "Set__ScnSystmPrFrmStt_m_nNumShadowedLightsInView",
    "Get__ScnSystmPrFrmStt_m_nNumShadowMaps",
    "Set__ScnSystmPrFrmStt_m_nNumShadowMaps",
    "Get__ScnSystmPrFrmStt_m_nNumRenderTargetBinds",
    "Set__ScnSystmPrFrmStt_m_nNumRenderTargetBinds",
    "Get__ScnSystmPrFrmStt_m_nPushConstantSets",
    "Set__ScnSystmPrFrmStt_m_nPushConstantSets",
    "ShaderTools_GetResourceVersion",
    "ShaderTools_GetVcsVersion",
    "ShaderTools_GetMinVcsVersion",
    "ShaderTools_GetAbiVersion",
    "ShaderTools_GetShaderSource",
    "ShaderTools_MaskShaderSource",
    "ShtSqncFrm_t_ImageCount",
    "ShtSqncFrm_t_GetImage",
    "Get__ShtSqncFrm_t_m_flDisplayTime",
    "Set__ShtSqncFrm_t_m_flDisplayTime",
    "SheetSequence_t_FrameCount",
    "Get__SheetSequence_t_m_nId",
    "Set__SheetSequence_t_m_nId",
    "Get__SheetSequence_t_m_bClamp",
    "Set__SheetSequence_t_m_bClamp",
    "Get__SheetSequence_t_m_bAlphaCrop",
    "Set__SheetSequence_t_m_bAlphaCrop",
    "Get__SheetSequence_t_m_bNoColor",
    "Set__SheetSequence_t_m_bNoColor",
    "Get__SheetSequence_t_m_bNoAlpha",
    "Set__SheetSequence_t_m_bNoAlpha",
    "Get__SheetSequence_t_m_flTotalTime",
    "Set__SheetSequence_t_m_flTotalTime",
    "Steam_Inventory_GetAllItems",
    "Steam_Inventory_DefinitionCount",
    "Steam_Inventory_GetDefinitionId",
    "Steam_Inventory_GetDefinitionProperty",
    "Steam_Inventory_GetDefinitionPrice",
    "Steam_Inventory_HasPrices",
    "Steam_Inventory_GetCurrency",
    "Steam_Inventory_CheckOut",
    "Steam_Inventory_IsCheckingOut",
    "Steam_Inventory_WasCheckoutSuccessful",
    "Steam_Inventory_Deserialize",
    "Steam_Screenshots_WriteScreenshot",
    "SteamUgc_CUgcQuery_CreateQuery",
    "SteamUgc_CUgcQuery_Dispose",
    "SteamUgc_CUgcQuery_GetResultJson",
    "Get__SteamUgc_CUgcQuery_m_complete",
    "Set__SteamUgc_CUgcQuery_m_complete",
    "Get__SteamUgc_CUgcQuery_m_success",
    "Set__SteamUgc_CUgcQuery_m_success",
    "Get__SteamUgc_CUgcQuery_m_resultCode",
    "Set__SteamUgc_CUgcQuery_m_resultCode",
    "SteamUgc_CgcSbscrptns_GetSubscribedJson",
    "SteamUgc_CgcSbscrptns_GetDownloadedJson",
    "SteamUgc_CgcSbscrptns_MarkDownloadedItemAsUnused",
    "SteamUgc_CgcSbscrptns_GetItemJson",
    "SteamUgc_CgcSbscrptns_DownloadItem",
    "SteamUgc_CgcSbscrptns_SubscribeItem",
    "SteamUgc_CgcSbscrptns_UnsubscribeItem",
    "SteamUgc_CgcSbscrptns_StartPlaytimeTracking",
    "SteamUgc_CgcSbscrptns_StopPlaytimeTracking",
    "SteamUgc_CgcSbscrptns_InitWorkshopForGameServer",
    "SteamUgc_CUgcUpdate_CreateCommunityItem",
    "SteamUgc_CUgcUpdate_CreateMtxItem",
    "SteamUgc_CUgcUpdate_OpenCommunityItem",
    "SteamUgc_CUgcUpdate_Dispose",
    "SteamUgc_CUgcUpdate_GetPublishedFileId",
    "SteamUgc_CUgcUpdate_SetTitle",
    "SteamUgc_CUgcUpdate_SetDescription",
    "SteamUgc_CUgcUpdate_SetLanguage",
    "SteamUgc_CUgcUpdate_SetMetadata",
    "SteamUgc_CUgcUpdate_SetVisibility",
    "SteamUgc_CUgcUpdate_SetTag",
    "SteamUgc_CUgcUpdate_SetContentFolder",
    "SteamUgc_CUgcUpdate_SetPreviewImage",
    "SteamUgc_CUgcUpdate_SetAllowLegacyUpload",
    "SteamUgc_CUgcUpdate_AddKeyValueTag",
    "SteamUgc_CUgcUpdate_RemoveKeyValueTags",
    "SteamUgc_CUgcUpdate_RemoveAllKeyValueTags",
    "SteamUgc_CUgcUpdate_AddPreviewFile",
    "SteamUgc_CUgcUpdate_AddPreviewVideo",
    "SteamUgc_CUgcUpdate_UpdatePreviewFile",
    "SteamUgc_CUgcUpdate_UpdatePreviewVideo",
    "SteamUgc_CUgcUpdate_RemovePreview",
    "SteamUgc_CUgcUpdate_SetRequiredGameVersions",
    "SteamUgc_CUgcUpdate_Submit",
    "SteamUgc_CUgcUpdate_GetProgressPercent",
    "SteamUgc_CUgcUpdate_GetBytesProcessed",
    "SteamUgc_CUgcUpdate_GetBytesTotal",
    "SteamUgc_CUgcUpdate_GetUpdateStatus",
    "Get__SteamUgc_CUgcUpdate_m_creating",
    "Set__SteamUgc_CUgcUpdate_m_creating",
    "Get__SteamUgc_CUgcUpdate_m_created",
    "Set__SteamUgc_CUgcUpdate_m_created",
    "Get__SteamUgc_CUgcUpdate_m_submitted",
    "Set__SteamUgc_CUgcUpdate_m_submitted",
    "Get__SteamUgc_CUgcUpdate_m_complete",
    "Set__SteamUgc_CUgcUpdate_m_complete",
    "Get__SteamUgc_CUgcUpdate_m_success",
    "Set__SteamUgc_CUgcUpdate_m_success",
    "Get__SteamUgc_CUgcUpdate_m_bNeedsLegalAgreement",
    "Set__SteamUgc_CUgcUpdate_m_bNeedsLegalAgreement",
    "Get__SteamUgc_CUgcUpdate_m_resultCode",
    "Set__SteamUgc_CUgcUpdate_m_resultCode",
    "SteamUgc_CUgcVote_GetVote",
    "SteamUgc_CUgcVote_SetVote",
    "SteamUgc_CUgcVote_Dispose",
    "Get__SteamUgc_CUgcVote_m_complete",
    "Set__SteamUgc_CUgcVote_m_complete",
    "Get__SteamUgc_CUgcVote_m_success",
    "Set__SteamUgc_CUgcVote_m_success",
    "Get__SteamUgc_CUgcVote_m_resultCode",
    "Set__SteamUgc_CUgcVote_m_resultCode",
    "Get__SteamUgc_CUgcVote_m_votedUp",
    "Set__SteamUgc_CUgcVote_m_votedUp",
    "Get__SteamUgc_CUgcVote_m_votedDown",
    "Set__SteamUgc_CUgcVote_m_votedDown",
    "VPhysXBodyPart_t_GetSphereCount",
    "VPhysXBodyPart_t_GetSphere",
    "VPhysXBodyPart_t_GetCapsuleCount",
    "VPhysXBodyPart_t_GetCapsule",
    "VPhysXBodyPart_t_GetHullCount",
    "VPhysXBodyPart_t_GetHull",
    "VPhysXBodyPart_t_GetMeshCount",
    "VPhysXBodyPart_t_GetMesh",
    "VPhysXBodyPart_t_GetCollisionAttributeCount",
    "VPhysXBodyPart_t_GetCollisionAttributeIndex",
    "Get__VPhysXBodyPart_t_m_nFlags",
    "Set__VPhysXBodyPart_t_m_nFlags",
    "Get__VPhysXBodyPart_t_m_flMass",
    "Set__VPhysXBodyPart_t_m_flMass",
    "Get__VPhysXBodyPart_t_m_nCollisionAttributeIndex",
    "Set__VPhysXBodyPart_t_m_nCollisionAttributeIndex",
    "Get__VPhysXBodyPart_t_m_flInertiaScale",
    "Set__VPhysXBodyPart_t_m_flInertiaScale",
    "Get__VPhysXBodyPart_t_m_flLinearDamping",
    "Set__VPhysXBodyPart_t_m_flLinearDamping",
    "Get__VPhysXBodyPart_t_m_flAngularDamping",
    "Set__VPhysXBodyPart_t_m_flAngularDamping",
    "Get__VPhysXBodyPart_t_m_flGravityScale",
    "Set__VPhysXBodyPart_t_m_flGravityScale",
    "Get__VPhysXBodyPart_t_m_bOverrideMassCenter",
    "Set__VPhysXBodyPart_t_m_bOverrideMassCenter",
    "Get__VPhysXBodyPart_t_m_vMassCenterOverride",
    "Set__VPhysXBodyPart_t_m_vMassCenterOverride",
    "VPhysXJoint_t_GetLinearLimitMin",
    "VPhysXJoint_t_GetLinearLimitMax",
    "VPhysXJoint_t_GetSwingLimitMin",
    "VPhysXJoint_t_GetSwingLimitMax",
    "VPhysXJoint_t_GetTwistLimitMin",
    "VPhysXJoint_t_GetTwistLimitMax",
    "VPhysXJoint_t_SetLinearLimitMin",
    "VPhysXJoint_t_SetLinearLimitMax",
    "VPhysXJoint_t_SetSwingLimitMin",
    "VPhysXJoint_t_SetSwingLimitMax",
    "VPhysXJoint_t_SetTwistLimitMin",
    "VPhysXJoint_t_SetTwistLimitMax",
    "Get__VPhysXJoint_t_m_nType",
    "Set__VPhysXJoint_t_m_nType",
    "Get__VPhysXJoint_t_m_nBody1",
    "Set__VPhysXJoint_t_m_nBody1",
    "Get__VPhysXJoint_t_m_nBody2",
    "Set__VPhysXJoint_t_m_nBody2",
    "Get__VPhysXJoint_t_m_nFlags",
    "Set__VPhysXJoint_t_m_nFlags",
    "Get__VPhysXJoint_t_m_bEnableCollision",
    "Set__VPhysXJoint_t_m_bEnableCollision",
    "Get__VPhysXJoint_t_m_bEnableLinearLimit",
    "Set__VPhysXJoint_t_m_bEnableLinearLimit",
    "Get__VPhysXJoint_t_m_bEnableLinearMotor",
    "Set__VPhysXJoint_t_m_bEnableLinearMotor",
    "Get__VPhysXJoint_t_m_vLinearTargetVelocity",
    "Set__VPhysXJoint_t_m_vLinearTargetVelocity",
    "Get__VPhysXJoint_t_m_flMaxForce",
    "Set__VPhysXJoint_t_m_flMaxForce",
    "Get__VPhysXJoint_t_m_bEnableSwingLimit",
    "Set__VPhysXJoint_t_m_bEnableSwingLimit",
    "Get__VPhysXJoint_t_m_bEnableTwistLimit",
    "Set__VPhysXJoint_t_m_bEnableTwistLimit",
    "Get__VPhysXJoint_t_m_bEnableAngularMotor",
    "Set__VPhysXJoint_t_m_bEnableAngularMotor",
    "Get__VPhysXJoint_t_m_vAngularTargetVelocity",
    "Set__VPhysXJoint_t_m_vAngularTargetVelocity",
    "Get__VPhysXJoint_t_m_flMaxTorque",
    "Set__VPhysXJoint_t_m_flMaxTorque",
    "Get__VPhysXJoint_t_m_flLinearFrequency",
    "Set__VPhysXJoint_t_m_flLinearFrequency",
    "Get__VPhysXJoint_t_m_flLinearDampingRatio",
    "Set__VPhysXJoint_t_m_flLinearDampingRatio",
    "Get__VPhysXJoint_t_m_flAngularFrequency",
    "Set__VPhysXJoint_t_m_flAngularFrequency",
    "Get__VPhysXJoint_t_m_flAngularDampingRatio",
    "Set__VPhysXJoint_t_m_flAngularDampingRatio",
    "Get__VPhysXJoint_t_m_flLinearStrength",
    "Set__VPhysXJoint_t_m_flLinearStrength",
    "Get__VPhysXJoint_t_m_flAngularStrength",
    "Set__VPhysXJoint_t_m_flAngularStrength",
    "Get__VPhysXJoint_t_m_bLinearMotorIsSpring",
    "Set__VPhysXJoint_t_m_bLinearMotorIsSpring",
    "Get__VPhysXJoint_t_m_bAngularMotorIsSpring",
    "Set__VPhysXJoint_t_m_bAngularMotorIsSpring",
    "Get__VPhysXJoint_t_m_flLinearTargetPosition",
    "Set__VPhysXJoint_t_m_flLinearTargetPosition",
    "Get__VPhysXJoint_t_m_flAngularTargetAngle",
    "Set__VPhysXJoint_t_m_flAngularTargetAngle",
    "Get__VPhysXJoint_t_m_qAngularTargetRotation",
    "Set__VPhysXJoint_t_m_qAngularTargetRotation",
    "Get__VPhysXJoint_t_m_bOverrideFriction",
    "Set__VPhysXJoint_t_m_bOverrideFriction",
    "Get__VPhysXJoint_t_m_flFriction",
    "Set__VPhysXJoint_t_m_flFriction",
    "Get__VPhysXJoint_t_m_Frame1",
    "Set__VPhysXJoint_t_m_Frame1",
    "Get__VPhysXJoint_t_m_Frame2",
    "Set__VPhysXJoint_t_m_Frame2",
    "VSound_t_DestroyStrongHandle",
    "VSound_t_IsStrongHandleValid",
    "VSound_t_IsError",
    "VSound_t_IsStrongHandleLoaded",
    "VSound_t_CopyStrongHandle",
    "VSound_t_GetBindingPtr",
    "VSound_t_GetGuid",
    "VSound_t_format",
    "VSound_t_BitsPerSample",
    "VSound_t_channels",
    "VSound_t_BytesPerSample",
    "VSound_t_m_sampleFrameSize",
    "VSound_t_m_rate",
    "VSound_t_Duration",
    "VertexLayout_Create",
    "VertexLayout_Destroy",
    "VertexLayout_Free",
    "VertexLayout_Add",
    "VertexLayout_Build",
    "Get__VertexLayout_m_Size",
    "Set__VertexLayout_m_Size",
    "VfxCmpldShdrnf_t_Delete",
    "VfxCmpldShdrnf_t_GetCompiledShader",
    "Get__VfxCmpldShdrnf_t_compilerOutput",
    "Set__VfxCmpldShdrnf_t_compilerOutput",
    "Get__VfxCmpldShdrnf_t_compileFailed",
    "Set__VfxCmpldShdrnf_t_compileFailed",
    "VoiceGlue_GetRecordingDeviceCount",
    "VoiceGlue_GetRecordingDeviceName",
    "VoiceGlue_GetDefaultRecordingDeviceName",
    "VoiceGlue_OpenCapture",
    "VoiceGlue_CloseCapture",
    "VoiceGlue_ReadCapture",
    "VoiceGlue_CreateEncoder",
    "VoiceGlue_DestroyEncoder",
    "VoiceGlue_Encode",
    "VoiceGlue_CreateDecoder",
    "VoiceGlue_DestroyDecoder",
    "VoiceGlue_Decode",
    "WindowsGlue_FindFile",
    "fpxr_pplctnCnfg_SetDebugCallback",
    "fpxr_pplctnCnfg_SetErrorCallback",
    "fpxr_Compositor_EventManager",
    "fpxr_Compositor_Submit",
    "fpxr_Compositor_BeginFrame",
    "fpxr_Compositor_EndFrame",
    "fpxr_Compositor_GetEyeWidth",
    "fpxr_Compositor_GetEyeHeight",
    "fpxr_Compositor_GetRenderTargetWidth",
    "fpxr_Compositor_GetRenderTargetHeight",
    "fpxr_Compositor_GetDisplayRefreshRate",
    "fpxr_Compositor_GetViewInfo",
    "fpxr_Compositor_GetProjectionMatrix",
    "fpxr_Compositor_Shutdown",
    "fpxr_EventManager_PumpEvent",
    "fpxr_EventManager_GetSessionState",
    "fpxr_Input_GetBooleanActionState",
    "fpxr_Input_GetFloatActionState",
    "fpxr_Input_GetVector2ActionState",
    "fpxr_Input_GetPoseActionState",
    "fpxr_Input_TriggerHapticVibration",
    "fpxr_Input_GetHandPoseState",
    "fpxr_Instance_Create",
    "fpxr_Instance_HasHeadset",
    "fpxr_Instance_Compositor",
    "fpxr_Instance_Input",
    "fpxr_Instance_GetRequiredDeviceExtensions",
    "fpxr_Instance_GetRequiredInstanceExtensions",
    "fpxr_Instance_GetProperties",
    "g_pnmtnSystmtls_MaintainDecodeCaches",
    "g_pAudioDevice_Name",
    "g_pAudioDevice_ChannelCount",
    "g_pAudioDevice_MixChannelCount",
    "g_pAudioDevice_BitsPerSample",
    "g_pAudioDevice_BytesPerSample",
    "g_pAudioDevice_SampleRate",
    "g_pAudioDevice_IsActive",
    "g_pAudioDevice_CancelOutput",
    "g_pAudioDevice_WaitForComplete",
    "g_pAudioDevice_MuteDevice",
    "g_pAudioDevice_ClearBuffer",
    "g_pAudioDevice_OutputDebugInfo",
    "g_pAudioDevice_IsValid",
    "g_pAudioDevice_SendOutput",
    "g_pngnPVSMngr_BuildPvs",
    "g_pngnPVSMngr_DestroyPvs",
    "g_pngnSrvcMgr_ExitMainLoop",
    "g_pFllFlSystm_GetSymLink",
    "g_pFllFlSystm_AddSymLink",
    "g_pFllFlSystm_RemoveSymLink",
    "g_pFllFlSystm_ResetProjectPaths",
    "g_pFllFlSystm_AddProjectPath",
    "g_pFllFlSystm_AddCloudPath",
    "g_pInputSystem_RegisterWindowWithSDL",
    "g_pInputSystem_UnregisterWindowFromSDL",
    "g_pInputSystem_SetEditorMainWindow",
    "g_pInputSystem_SetGameFocus",
    "g_pMtrlSystm2_CreateRawMaterial",
    "g_pMtrlSystm2_CreateRawMaterialFromShader",
    "g_pMtrlSystm2_CreateProceduralMaterialCopy",
    "g_pMtrlSystm2_FindOrCreateMaterialFromResource",
    "g_pMtrlSystm2_FrameUpdate",
    "g_pMtrlSystm2tls_DrawScreenSpaceRectangle",
    "g_pMeshSystem_CreateSceneObject",
    "g_pMeshSystem_ChangeModel",
    "g_pPhysicsSystem_NumWorlds",
    "g_pPhysicsSystem_CreateWorld",
    "g_pPhysicsSystem_DestroyWorld",
    "g_pPhysicsSystem_GetSurfacePropertyController",
    "g_pPhysicsSystem_CastHeightField",
    "g_pPhysicsSystem_GetAggregateData",
    "g_pPhysicsSystem_UpdateSurfaceProperties",
    "g_pRenderDevice_FindOrCreateSamplerState",
    "g_pRenderDevice_GetSamplerIndex",
    "g_pRenderDevice_GetSwapChainInfo",
    "g_pRenderDevice_FindOrCreateFileTexture",
    "g_pRenderDevice_FindOrCreateTexture2",
    "g_pRenderDevice_ClearTexture",
    "g_pRenderDevice_AsyncSetTextureData2",
    "g_pRenderDevice_GetSwapChainTexture",
    "g_pRenderDevice_GetSheetMetaDataTexture",
    "g_pRenderDevice_GetGPUFrameTimeMS",
    "g_pRenderDevice_GetTextureDesc",
    "g_pRenderDevice_GetOnDiskTextureDesc",
    "g_pRenderDevice_GetTextureMultisampleType",
    "g_pRenderDevice_ComputeTextureMemorySize",
    "g_pRenderDevice_CreateRenderContext",
    "g_pRenderDevice_ReleaseRenderContext",
    "g_pRenderDevice_BeginSubmittingDisplayLists",
    "g_pRenderDevice_BeginSubmittingDisplayLists_1",
    "g_pRenderDevice_ReadTexturePixels",
    "g_pRenderDevice_CreateSwapChain",
    "g_pRenderDevice_UpdateSwapChain",
    "g_pRenderDevice_DestroySwapChain",
    "g_pRenderDevice_GetGpuStatsSummary",
    "g_pRenderDevice_GetTexturePoolStats",
    "g_pRenderDevice_Present",
    "g_pRenderDevice_Flush",
    "g_pRenderDevice_FlushPipelineCache",
    "g_pRenderDevice_ForceFlushGPU",
    "g_pRenderDevice_CanRenderToSwapChain",
    "g_pRenderDevice_SetSwapChainIsMainWindow",
    "g_pRenderDevice_IsUsing32BitDepthBuffer",
    "g_pRenderDevice_GetBackbufferDimensions",
    "g_pRenderDevice_GetTextureLastUsed",
    "g_pRenderDevice_UnThrottleTextureStreamingForNFrames",
    "g_pRenderDevice_GetNumTextureLoadsInFlight",
    "g_pRenderDevice_SetForcePreloadStreamingData",
    "g_pRenderDevice_GetRenderDeviceAPI",
    "g_pRenderDevice_MarkTextureUsed",
    "g_pRenderDevice_IsTextureRenderTarget",
    "g_pRenderDevice_IsRayTracingSupported",
    "g_pRenderDevice_AreDebugMarkersEnabled",
    "g_pRenderDevice_CreateGPUBuffer",
    "g_pRenderDevice_DestroyGPUBuffer",
    "g_pRenderDevice_ReadBuffer",
    "g_pRenderDevice_GetDeviceSpecificInfo",
    "g_pRenderDevice_GetGraphicsAPISpecificTextureHandle",
    "g_pRenderDevice_GetDeviceSpecificTexture",
    "g_pRenderDevice_GetTextureViewIndex",
    "g_pRenderDevice_GetVideoMemoryInfo",
    "g_pRenderDevice_GetTextureResidencyInfo",
    "g_pRenderDevice_GetSheetInfo",
    "g_pRenderDevice_GetSequenceCount",
    "g_pRenderDevice_GetSequence",
    "g_pRsrcCmplrSyst_GenerateResourceFile",
    "g_pRsrcCmplrSyst_GenerateResourceFile_1",
    "g_pRsrcCmplrSyst_GenerateResourceBytes",
    "g_pRsrcCmplrSyst_GenerateResourceFileForced",
    "g_pRsrcSystm_ReloadSymlinkedResidentResources",
    "g_pRsrcSystm_UpdateSimple",
    "g_pRsrcSystm_LoadTexture",
    "g_pRsrcSystm_HasPendingWork",
    "g_pRsrcSystm_GetNumPendingStreamingRequests",
    "g_pRsrcSystm_IsShuttingDown",
    "g_pRsrcSystm_InvalidateDatabase",
    "g_pRsrcSystm_NotifyMetaFileChanged",
    "g_pRsrcSystm_LoadResourceInManifest",
    "g_pRsrcSystm_ReloadResource",
    "g_pRsrcSystm_DestroyResourceManifest",
    "g_pRsrcSystm_IsManifestLoaded",
    "g_pRsrcSystm_GetAllCodeManifests",
    "g_pRsrcSystm_ResolvePathToGuid",
    "g_pRsrcSystm_NeedsGuidWrite",
    "g_pSceneSystem_FrameUpdate",
    "g_pSceneSystem_SetMainSwapChain",
    "g_pSceneSystem_GetMainSwapChainMultisampleType",
    "g_pSceneSystem_DeleteSceneObject",
    "g_pSceneSystem_DeleteSceneObjectAtFrameEnd",
    "g_pSceneSystem_CreateSkyBox",
    "g_pSceneSystem_CreateDecal",
    "g_pSceneSystem_BeginRenderingDynamicView",
    "g_pSceneSystem_AddShadowView",
    "g_pSceneSystem_AddShadowView_1",
    "g_pSceneSystem_GetWellKnownTexture",
    "g_pSceneSystem_GetWellKnownMaterialHandle",
    "g_pSceneSystem_GetPerFrameStats",
    "g_pSceneSystem_GetNumUnbatchableMaterials",
    "g_pSceneSystem_CreateWorld",
    "g_pSceneSystem_DestroyWorld",
    "g_pSceneSystem_PerformEndOfFrameDeletes",
    "g_pSceneSystem_BeginRenderingViews",
    "g_pSceneSystem_FinishRenderingViews",
    "g_pSceneSystem_WaitForRenderingToComplete",
    "g_pSceneSystem_IsRenderingBusy",
    "g_pSceneSystem_IsRenderingViews",
    "g_pSceneSystem_SetupPerObjectLighting",
    "g_pSceneSystem_CreatePointLight",
    "g_pSceneSystem_CreateSpotLight",
    "g_pSceneSystem_CreateOrthoLight",
    "g_pSceneSystem_CreateDirectionalLight",
    "g_pSceneSystem_CreateEnvMap",
    "g_pSceneSystem_CreateLightProbeVolume",
    "g_pSceneSystem_MarkEnvironmentMapObjectUpdated",
    "g_pSceneSystem_MarkLightProbeVolumeObjectUpdated",
    "g_pSceneSystem_AddCullingBox",
    "g_pSceneSystem_RemoveCullingBox",
    "g_pSceneSystem_AddVolumetricFogVolume",
    "g_pSceneSystem_RemoveVolumetricFogVolume",
    "g_pSceneSystem_DownsampleTexture",
    "g_pSceneSystem_BindTransformSlot",
    "g_pSceneSystem_SetPipelineTextureIndex",
    "g_pSceneSystem_UploadPipelineTextureIndices",
    "g_pSceneSystem_CreateRayTraceWorld",
    "g_pSceneSystem_DestroyRayTraceWorld",
    "g_pSceneSystem_SetGPUProfilerMode",
    "g_pSceneSystem_GetGPUProfilerMode",
    "g_pSceneSystem_RefreshGpuTimestampSnapshot",
    "g_pSceneSystem_GetGpuTimestampCount",
    "g_pSceneSystem_GetGpuTimestampRenderer",
    "g_pSceneSystem_GetGpuTimestampFrameId",
    "g_pSceneSystem_GetGpuTimestampDuration",
    "g_pSceneSystem_GetGpuTimestampName",
    "g_pSceneSystem_GetGpuTimestampParent",
    "g_pSceneSystem_GetGpuTimestampStableId",
    "g_pSceneSystem_GetGpuTimestampMeasured",
    "g_pSceneSystem_GetGpuTimestampUnparented",
    "g_pSceneSystem_GetGpuTimestampEventCount",
    "g_pSceneSystem_GetGpuTimestampEvent",
    "g_pSceneSystem_BeginManagedPerfMarker",
    "g_pSceneSystem_EndManagedPerfMarker",
    "g_pSceneSystem_SetNextRenderTime",
    "g_pSceneSystem_GetCurrentRenderTime",
    "g_pSceneUtils_CreateTonemapSystem",
    "g_pSceneUtils_DestroyTonemapSystem",
    "g_pSceneUtils_CreateVolumetricFog",
    "g_pSceneUtils_DestroyVolumetricFog",
    "g_pSndSystmntrnl_GetNumAudioDevices",
    "g_pSndSystmntrnl_GetAudioDeviceDesc",
    "g_pSndSystmntrnl_GetAudioDeviceId",
    "g_pSndSystmntrnl_GetAudioDeviceName",
    "g_pSndSystmntrnl_GetActiveAudioDevice",
    "g_pSndSystmntrnl_SetActiveAudioDevice",
    "g_pSndSystmntrnl_PlaySoundAtOSLevel",
    "g_pSndSystmntrnl_PrecacheSound",
    "g_pSndSystmntrnl_CreateSound",
    "g_pSndSystmntrnl_PreloadSound",
    "g_pWrldRndrrMgr_ServiceWorldRequests",
    "g_pWrldRndrrMgr_UpdateObjectsForRendering",
    "g_pWrldRndrrMgr_CreateWorld",
    "g_pWrldRndrrMgr_MountWorldVPK",
    "g_pWrldRndrrMgr_UnmountWorldVPK",
    "global_Plat_ScreenToWindowCoords",
    "global_Plat_WindowToScreenCoords",
    "global_Plat_MessageBox",
    "global_Plat_SafeRemoveFile",
    "global_Plat_SetModuleFilename",
    "global_Plat_SetCurrentDirectory",
    "global_Plat_GetCurrentFrame",
    "global_Plat_SetCurrentFrame",
    "global_Plat_ChangeCurrentFrame",
    "global_Plat_IsRunningOnCustomerMachine",
    "global_IsRetail",
    "global_HasLaunchParameter",
    "global_Plat_SetNoAssert",
    "global_Plat_ExitProcess",
    "global_GetGameRootFolder",
    "global_GetGameSearchPath",
    "global_SourceEngineUnitTestInit",
    "global_SourceEngineInitStartupWindow",
    "global_SourceEnginePreInit",
    "global_SourceEngineInit",
    "global_SourceEnginePanelAppInit",
    "global_SourceEnginePanelAppFrame",
    "global_SourceEngineFrame",
    "global_SourceEngineShutdown",
    "global_AppIsDedicatedServer",
    "global_ToolsStallMonitor_IndicateActivity",
    "globalBox2d_b2CreateWorld",
    "globalBox2d_b2CreateBody",
    "globalBox2d_b2CreateCircleShape",
    "globalBox2d_b2CreateSegmentShape",
    "globalBox2d_b2CreateCapsuleShape",
    "globalBox2d_b2CreatePolygonShape",
    "globalBox2d_b2CreateChain",
    "globalBox2d_b2CreateDistanceJoint",
    "globalBox2d_b2CreateMotorJoint",
    "globalBox2d_b2CreateFilterJoint",
    "globalBox2d_b2CreatePrismaticJoint",
    "globalBox2d_b2CreateRevoluteJoint",
    "globalBox2d_b2CreateWeldJoint",
    "globalBox2d_b2CreateWheelJoint",
    "globalBox2d_b2World_Explode",
    "globalBox2d_b2DestroyWorld",
    "globalBox2d_b2SetLengthUnitsPerMeter",
    "globalBox2d_b2GetLengthUnitsPerMeter",
    "globalBox2d_b2World_IsValid",
    "globalBox2d_b2World_Step",
    "globalBox2d_b2World_Draw",
    "globalBox2d_b2World_GetBodyEvents",
    "globalBox2d_b2World_GetSensorEvents",
    "globalBox2d_b2World_GetContactEvents",
    "globalBox2d_b2World_GetJointEvents",
    "globalBox2d_b2World_OverlapAABB",
    "globalBox2d_b2World_OverlapShape",
    "globalBox2d_b2World_CastRay",
    "globalBox2d_b2World_CastRayClosest",
    "globalBox2d_b2World_CastShape",
    "globalBox2d_b2World_CastMover",
    "globalBox2d_b2World_CollideMover",
    "globalBox2d_b2World_EnableSleeping",
    "globalBox2d_b2World_IsSleepingEnabled",
    "globalBox2d_b2World_EnableContinuous",
    "globalBox2d_b2World_IsContinuousEnabled",
    "globalBox2d_b2World_SetRestitutionThreshold",
    "globalBox2d_b2World_GetRestitutionThreshold",
    "globalBox2d_b2World_SetHitEventThreshold",
    "globalBox2d_b2World_GetHitEventThreshold",
    "globalBox2d_b2World_SetCustomFilterCallback",
    "globalBox2d_b2World_SetPreSolveCallback",
    "globalBox2d_b2World_SetGravity",
    "globalBox2d_b2World_GetGravity",
    "globalBox2d_b2World_SetContactTuning",
    "globalBox2d_b2World_SetContactRecycleDistance",
    "globalBox2d_b2World_GetContactRecycleDistance",
    "globalBox2d_b2World_SetMaximumLinearSpeed",
    "globalBox2d_b2World_GetMaximumLinearSpeed",
    "globalBox2d_b2World_EnableWarmStarting",
    "globalBox2d_b2World_IsWarmStartingEnabled",
    "globalBox2d_b2World_GetAwakeBodyCount",
    "globalBox2d_b2World_GetProfile",
    "globalBox2d_b2World_GetCounters",
    "globalBox2d_b2World_SetUserData",
    "globalBox2d_b2World_GetUserData",
    "globalBox2d_b2World_SetFrictionCallback",
    "globalBox2d_b2World_SetRestitutionCallback",
    "globalBox2d_b2World_DumpMemoryStats",
    "globalBox2d_b2World_RebuildStaticTree",
    "globalBox2d_b2World_EnableSpeculative",
    "globalBox2d_b2World_GetMaxCapacity",
    "globalBox2d_b2World_GetWorkerCount",
    "globalBox2d_b2World_SetWorkerCount",
    "globalBox2d_b2DestroyBody",
    "globalBox2d_b2Body_IsValid",
    "globalBox2d_b2Body_GetType",
    "globalBox2d_b2Body_SetType",
    "globalBox2d_b2Body_SetName",
    "globalBox2d_b2Body_GetName",
    "globalBox2d_b2Body_SetUserData",
    "globalBox2d_b2Body_GetUserData",
    "globalBox2d_b2Body_GetPosition",
    "globalBox2d_b2Body_GetRotation",
    "globalBox2d_b2Body_GetTransform",
    "globalBox2d_b2Body_SetTransform",
    "globalBox2d_b2Body_GetLocalPoint",
    "globalBox2d_b2Body_GetWorldPoint",
    "globalBox2d_b2Body_GetLocalVector",
    "globalBox2d_b2Body_GetWorldVector",
    "globalBox2d_b2Body_GetLinearVelocity",
    "globalBox2d_b2Body_GetAngularVelocity",
    "globalBox2d_b2Body_SetLinearVelocity",
    "globalBox2d_b2Body_SetAngularVelocity",
    "globalBox2d_b2Body_SetTargetTransform",
    "globalBox2d_b2Body_GetLocalPointVelocity",
    "globalBox2d_b2Body_GetWorldPointVelocity",
    "globalBox2d_b2Body_ApplyForce",
    "globalBox2d_b2Body_ApplyForceToCenter",
    "globalBox2d_b2Body_ApplyTorque",
    "globalBox2d_b2Body_ClearForces",
    "globalBox2d_b2Body_ApplyLinearImpulse",
    "globalBox2d_b2Body_ApplyLinearImpulseToCenter",
    "globalBox2d_b2Body_ApplyAngularImpulse",
    "globalBox2d_b2Body_GetMass",
    "globalBox2d_b2Body_GetRotationalInertia",
    "globalBox2d_b2Body_GetLocalCenterOfMass",
    "globalBox2d_b2Body_GetWorldCenterOfMass",
    "globalBox2d_b2Body_SetMassData",
    "globalBox2d_b2Body_GetMassData",
    "globalBox2d_b2Body_ApplyMassFromShapes",
    "globalBox2d_b2Body_SetLinearDamping",
    "globalBox2d_b2Body_GetLinearDamping",
    "globalBox2d_b2Body_SetAngularDamping",
    "globalBox2d_b2Body_GetAngularDamping",
    "globalBox2d_b2Body_SetGravityScale",
    "globalBox2d_b2Body_GetGravityScale",
    "globalBox2d_b2Body_IsAwake",
    "globalBox2d_b2Body_SetAwake",
    "globalBox2d_b2Body_WakeTouching",
    "globalBox2d_b2Body_EnableSleep",
    "globalBox2d_b2Body_IsSleepEnabled",
    "globalBox2d_b2Body_SetSleepThreshold",
    "globalBox2d_b2Body_GetSleepThreshold",
    "globalBox2d_b2Body_IsEnabled",
    "globalBox2d_b2Body_Disable",
    "globalBox2d_b2Body_Enable",
    "globalBox2d_b2Body_SetMotionLocks",
    "globalBox2d_b2Body_GetMotionLocks",
    "globalBox2d_b2Body_SetBullet",
    "globalBox2d_b2Body_IsBullet",
    "globalBox2d_b2Body_EnableContactEvents",
    "globalBox2d_b2Body_EnableHitEvents",
    "globalBox2d_b2Body_GetWorld",
    "globalBox2d_b2Body_GetShapeCount",
    "globalBox2d_b2Body_GetShapes",
    "globalBox2d_b2Body_GetJointCount",
    "globalBox2d_b2Body_GetJoints",
    "globalBox2d_b2Body_GetContactCapacity",
    "globalBox2d_b2Body_GetContactData",
    "globalBox2d_b2Body_ComputeAABB",
    "globalBox2d_b2DestroyShape",
    "globalBox2d_b2Shape_IsValid",
    "globalBox2d_b2Shape_GetType",
    "globalBox2d_b2Shape_GetBody",
    "globalBox2d_b2Shape_GetWorld",
    "globalBox2d_b2Shape_IsSensor",
    "globalBox2d_b2Shape_SetUserData",
    "globalBox2d_b2Shape_GetUserData",
    "globalBox2d_b2Shape_SetDensity",
    "globalBox2d_b2Shape_GetDensity",
    "globalBox2d_b2Shape_SetFriction",
    "globalBox2d_b2Shape_GetFriction",
    "globalBox2d_b2Shape_SetRestitution",
    "globalBox2d_b2Shape_GetRestitution",
    "globalBox2d_b2Shape_SetUserMaterial",
    "globalBox2d_b2Shape_GetUserMaterial",
    "globalBox2d_b2Shape_SetSurfaceMaterial",
    "globalBox2d_b2Shape_GetSurfaceMaterial",
    "globalBox2d_b2Shape_GetFilter",
    "globalBox2d_b2Shape_SetFilter",
    "globalBox2d_b2Shape_EnableSensorEvents",
    "globalBox2d_b2Shape_AreSensorEventsEnabled",
    "globalBox2d_b2Shape_EnableContactEvents",
    "globalBox2d_b2Shape_AreContactEventsEnabled",
    "globalBox2d_b2Shape_EnablePreSolveEvents",
    "globalBox2d_b2Shape_ArePreSolveEventsEnabled",
    "globalBox2d_b2Shape_EnableHitEvents",
    "globalBox2d_b2Shape_AreHitEventsEnabled",
    "globalBox2d_b2Shape_TestPoint",
    "globalBox2d_b2Shape_RayCast",
    "globalBox2d_b2Shape_GetCircle",
    "globalBox2d_b2Shape_GetSegment",
    "globalBox2d_b2Shape_GetChainSegment",
    "globalBox2d_b2Shape_GetCapsule",
    "globalBox2d_b2Shape_GetPolygon",
    "globalBox2d_b2Shape_SetCircle",
    "globalBox2d_b2Shape_SetCapsule",
    "globalBox2d_b2Shape_SetSegment",
    "globalBox2d_b2Shape_SetPolygon",
    "globalBox2d_b2Shape_GetParentChain",
    "globalBox2d_b2Shape_GetContactCapacity",
    "globalBox2d_b2Shape_GetContactData",
    "globalBox2d_b2Shape_GetSensorCapacity",
    "globalBox2d_b2Shape_GetSensorData",
    "globalBox2d_b2Shape_GetAABB",
    "globalBox2d_b2Shape_ComputeMassData",
    "globalBox2d_b2Shape_GetClosestPoint",
    "globalBox2d_b2Shape_ApplyWind",
    "globalBox2d_b2DestroyChain",
    "globalBox2d_b2Chain_GetWorld",
    "globalBox2d_b2Chain_GetSegmentCount",
    "globalBox2d_b2Chain_GetSegments",
    "globalBox2d_b2Chain_GetSurfaceMaterialCount",
    "globalBox2d_b2Chain_SetSurfaceMaterial",
    "globalBox2d_b2Chain_GetSurfaceMaterial",
    "globalBox2d_b2Chain_IsValid",
    "globalBox2d_b2DestroyJoint",
    "globalBox2d_b2Joint_IsValid",
    "globalBox2d_b2Joint_GetType",
    "globalBox2d_b2Joint_GetBodyA",
    "globalBox2d_b2Joint_GetBodyB",
    "globalBox2d_b2Joint_GetWorld",
    "globalBox2d_b2Joint_SetLocalFrameA",
    "globalBox2d_b2Joint_GetLocalFrameA",
    "globalBox2d_b2Joint_SetLocalFrameB",
    "globalBox2d_b2Joint_GetLocalFrameB",
    "globalBox2d_b2Joint_SetCollideConnected",
    "globalBox2d_b2Joint_GetCollideConnected",
    "globalBox2d_b2Joint_SetUserData",
    "globalBox2d_b2Joint_GetUserData",
    "globalBox2d_b2Joint_WakeBodies",
    "globalBox2d_b2Joint_GetConstraintForce",
    "globalBox2d_b2Joint_GetConstraintTorque",
    "globalBox2d_b2Joint_GetLinearSeparation",
    "globalBox2d_b2Joint_GetAngularSeparation",
    "globalBox2d_b2Joint_SetConstraintTuning",
    "globalBox2d_b2Joint_GetConstraintTuning",
    "globalBox2d_b2Joint_SetForceThreshold",
    "globalBox2d_b2Joint_GetForceThreshold",
    "globalBox2d_b2Joint_SetTorqueThreshold",
    "globalBox2d_b2Joint_GetTorqueThreshold",
    "globalBox2d_b2DistanceJoint_SetLength",
    "globalBox2d_b2DistanceJoint_GetLength",
    "globalBox2d_b2DistanceJoint_EnableSpring",
    "globalBox2d_b2DistanceJoint_IsSpringEnabled",
    "globalBox2d_b2DistanceJoint_SetSpringForceRange",
    "globalBox2d_b2DistanceJoint_GetSpringForceRange",
    "globalBox2d_b2DistanceJoint_SetSpringHertz",
    "globalBox2d_b2DistanceJoint_SetSpringDampingRatio",
    "globalBox2d_b2DistanceJoint_GetSpringHertz",
    "globalBox2d_b2DistanceJoint_GetSpringDampingRatio",
    "globalBox2d_b2DistanceJoint_EnableLimit",
    "globalBox2d_b2DistanceJoint_IsLimitEnabled",
    "globalBox2d_b2DistanceJoint_SetLengthRange",
    "globalBox2d_b2DistanceJoint_GetMinLength",
    "globalBox2d_b2DistanceJoint_GetMaxLength",
    "globalBox2d_b2DistanceJoint_GetCurrentLength",
    "globalBox2d_b2DistanceJoint_EnableMotor",
    "globalBox2d_b2DistanceJoint_IsMotorEnabled",
    "globalBox2d_b2DistanceJoint_SetMotorSpeed",
    "globalBox2d_b2DistanceJoint_GetMotorSpeed",
    "globalBox2d_b2DistanceJoint_SetMaxMotorForce",
    "globalBox2d_b2DistanceJoint_GetMaxMotorForce",
    "globalBox2d_b2DistanceJoint_GetMotorForce",
    "globalBox2d_b2MotorJoint_SetLinearVelocity",
    "globalBox2d_b2MotorJoint_GetLinearVelocity",
    "globalBox2d_b2MotorJoint_SetAngularVelocity",
    "globalBox2d_b2MotorJoint_GetAngularVelocity",
    "globalBox2d_b2MotorJoint_SetMaxVelocityForce",
    "globalBox2d_b2MotorJoint_GetMaxVelocityForce",
    "globalBox2d_b2MotorJoint_SetMaxVelocityTorque",
    "globalBox2d_b2MotorJoint_GetMaxVelocityTorque",
    "globalBox2d_b2MotorJoint_SetLinearHertz",
    "globalBox2d_b2MotorJoint_GetLinearHertz",
    "globalBox2d_b2MotorJoint_SetLinearDampingRatio",
    "globalBox2d_b2MotorJoint_GetLinearDampingRatio",
    "globalBox2d_b2MotorJoint_SetAngularHertz",
    "globalBox2d_b2MotorJoint_GetAngularHertz",
    "globalBox2d_b2MotorJoint_SetAngularDampingRatio",
    "globalBox2d_b2MotorJoint_GetAngularDampingRatio",
    "globalBox2d_b2MotorJoint_SetMaxSpringForce",
    "globalBox2d_b2MotorJoint_GetMaxSpringForce",
    "globalBox2d_b2MotorJoint_SetMaxSpringTorque",
    "globalBox2d_b2MotorJoint_GetMaxSpringTorque",
    "globalBox2d_b2PrismaticJoint_EnableSpring",
    "globalBox2d_b2PrismaticJoint_IsSpringEnabled",
    "globalBox2d_b2PrismaticJoint_SetSpringHertz",
    "globalBox2d_b2PrismaticJoint_GetSpringHertz",
    "globalBox2d_b2PrismaticJoint_SetSpringDampingRatio",
    "globalBox2d_b2PrismaticJoint_GetSpringDampingRatio",
    "globalBox2d_b2PrismaticJoint_SetTargetTranslation",
    "globalBox2d_b2PrismaticJoint_GetTargetTranslation",
    "globalBox2d_b2PrismaticJoint_EnableLimit",
    "globalBox2d_b2PrismaticJoint_IsLimitEnabled",
    "globalBox2d_b2PrismaticJoint_GetLowerLimit",
    "globalBox2d_b2PrismaticJoint_GetUpperLimit",
    "globalBox2d_b2PrismaticJoint_SetLimits",
    "globalBox2d_b2PrismaticJoint_EnableMotor",
    "globalBox2d_b2PrismaticJoint_IsMotorEnabled",
    "globalBox2d_b2PrismaticJoint_SetMotorSpeed",
    "globalBox2d_b2PrismaticJoint_GetMotorSpeed",
    "globalBox2d_b2PrismaticJoint_SetMaxMotorForce",
    "globalBox2d_b2PrismaticJoint_GetMaxMotorForce",
    "globalBox2d_b2PrismaticJoint_GetMotorForce",
    "globalBox2d_b2PrismaticJoint_GetTranslation",
    "globalBox2d_b2PrismaticJoint_GetSpeed",
    "globalBox2d_b2RevoluteJoint_EnableSpring",
    "globalBox2d_b2RevoluteJoint_IsSpringEnabled",
    "globalBox2d_b2RevoluteJoint_SetSpringHertz",
    "globalBox2d_b2RevoluteJoint_GetSpringHertz",
    "globalBox2d_b2RevoluteJoint_SetSpringDampingRatio",
    "globalBox2d_b2RevoluteJoint_GetSpringDampingRatio",
    "globalBox2d_b2RevoluteJoint_SetTargetAngle",
    "globalBox2d_b2RevoluteJoint_GetTargetAngle",
    "globalBox2d_b2RevoluteJoint_GetAngle",
    "globalBox2d_b2RevoluteJoint_EnableLimit",
    "globalBox2d_b2RevoluteJoint_IsLimitEnabled",
    "globalBox2d_b2RevoluteJoint_GetLowerLimit",
    "globalBox2d_b2RevoluteJoint_GetUpperLimit",
    "globalBox2d_b2RevoluteJoint_SetLimits",
    "globalBox2d_b2RevoluteJoint_EnableMotor",
    "globalBox2d_b2RevoluteJoint_IsMotorEnabled",
    "globalBox2d_b2RevoluteJoint_SetMotorSpeed",
    "globalBox2d_b2RevoluteJoint_GetMotorSpeed",
    "globalBox2d_b2RevoluteJoint_GetMotorTorque",
    "globalBox2d_b2RevoluteJoint_SetMaxMotorTorque",
    "globalBox2d_b2RevoluteJoint_GetMaxMotorTorque",
    "globalBox2d_b2WeldJoint_SetLinearHertz",
    "globalBox2d_b2WeldJoint_GetLinearHertz",
    "globalBox2d_b2WeldJoint_SetLinearDampingRatio",
    "globalBox2d_b2WeldJoint_GetLinearDampingRatio",
    "globalBox2d_b2WeldJoint_SetAngularHertz",
    "globalBox2d_b2WeldJoint_GetAngularHertz",
    "globalBox2d_b2WeldJoint_SetAngularDampingRatio",
    "globalBox2d_b2WeldJoint_GetAngularDampingRatio",
    "globalBox2d_b2WheelJoint_EnableSpring",
    "globalBox2d_b2WheelJoint_IsSpringEnabled",
    "globalBox2d_b2WheelJoint_SetSpringHertz",
    "globalBox2d_b2WheelJoint_GetSpringHertz",
    "globalBox2d_b2WheelJoint_SetSpringDampingRatio",
    "globalBox2d_b2WheelJoint_GetSpringDampingRatio",
    "globalBox2d_b2WheelJoint_EnableLimit",
    "globalBox2d_b2WheelJoint_IsLimitEnabled",
    "globalBox2d_b2WheelJoint_GetLowerLimit",
    "globalBox2d_b2WheelJoint_GetUpperLimit",
    "globalBox2d_b2WheelJoint_SetLimits",
    "globalBox2d_b2WheelJoint_EnableMotor",
    "globalBox2d_b2WheelJoint_IsMotorEnabled",
    "globalBox2d_b2WheelJoint_SetMotorSpeed",
    "globalBox2d_b2WheelJoint_GetMotorSpeed",
    "globalBox2d_b2WheelJoint_SetMaxMotorTorque",
    "globalBox2d_b2WheelJoint_GetMaxMotorTorque",
    "globalBox2d_b2WheelJoint_GetMotorTorque",
    "globalBox2d_b2Contact_IsValid",
    "globalBox2d_b2Contact_GetData",
    "globalBox2d_b2DefaultWorldDef",
    "globalBox2d_b2DefaultBodyDef",
    "globalBox2d_b2DefaultShapeDef",
    "globalBox2d_b2DefaultChainDef",
    "globalBox2d_b2DefaultDistanceJointDef",
    "globalBox2d_b2DefaultMotorJointDef",
    "globalBox2d_b2DefaultFilterJointDef",
    "globalBox2d_b2DefaultPrismaticJointDef",
    "globalBox2d_b2DefaultRevoluteJointDef",
    "globalBox2d_b2DefaultWeldJointDef",
    "globalBox2d_b2DefaultWheelJointDef",
    "globalBox2d_b2DefaultExplosionDef",
    "globalBox2d_b2DefaultFilter",
    "globalBox2d_b2DefaultQueryFilter",
    "globalBox2d_b2DefaultSurfaceMaterial",
    "globalBox2d_b2MakeBox",
    "globalBox2d_b2MakeOffsetBox",
    "globalBox2d_b2ComputeHull",
    "globalBox2d_b2MakePolygon",
    "globalBox2d_b2MakeOffsetPolygon",
    "globalOVRLipSync_ovrLipSync_Initialize",
    "globalOVRLipSync_ovrLipSync_Shutdown",
    "globalOVRLipSync_ovrLipSync_DestroyContext",
    "globalOVRLipSync_ovrLipSync_ResetContext",
    "globalOVRLipSync_ovrLipSync_CreateContext",
    "globalOVRLipSync_ovrLipSync_CreateContextEx",
    "globalOVRLipSync_ovrLipSync_SendSignal",
    "globalOVRLipSync_ovrLipSync_ProcessFrameEx",
    "globalSdl_DelayPrecise",
    "globalSdl_HasClipboardText",
    "globalSdl_SetClipboardText",
    "globalSdl_GetClipboardText",
    "globalSdl_Free",
    "globalSdl_GetError",
    "globalSdl_GetDisplays",
    "globalSdl_GetPrimaryDisplay",
    "globalSdl_GetDisplayForPoint",
    "globalSdl_GetDisplayContentScale",
    "globalSdl_GetDisplayBounds",
    "globalSdl_GetDisplayUsableBounds",
    "globalSdl_GetDesktopDisplayMode",
    "globalSdl_GetCurrentDisplayMode",
    "globalSdl_GetFullscreenDisplayModes",
    "globalSdl_GetClosestFullscreenDisplayMode",
    "globalSdl_GetCurrentVideoDriver",
    "globalSdl_CreateProperties",
    "globalSdl_DestroyProperties",
    "globalSdl_GetWindowProperties",
    "globalSdl_GetPointerProperty",
    "globalSdl_SetStringProperty",
    "globalSdl_SetNumberProperty",
    "globalSdl_SetBooleanProperty",
    "globalSdl_GetBooleanProperty",
    "globalSdl_CreateWindowWithProperties",
    "globalSdl_GetDisplayForWindow",
    "globalSdl_SetWindowFullscreenMode",
    "globalSdl_CreateWindow",
    "globalSdl_CreateStartupWindow",
    "globalSdl_DestroyWindow",
    "globalSdl_SetHint",
    "globalSdl_GetWindowPosition",
    "globalSdl_SetWindowTitle",
    "globalSdl_ShowWindow",
    "globalSdl_HideWindow",
    "globalSdl_MinimizeWindow",
    "globalSdl_MaximizeWindow",
    "globalSdl_RestoreWindow",
    "globalSdl_RaiseWindow",
    "globalSdl_GetWindowSize",
    "globalSdl_GetWindowPixelDensity",
    "globalSdl_SetWindowMinimumSize",
    "globalSdl_SetWindowMaximumSize",
    "globalSdl_SetWindowAlwaysOnTop",
    "globalSdl_SetWindowOpacity",
    "globalSdl_SetWindowAspectRatio",
    "globalSdl_SetWindowParent",
    "globalSdl_SetWindowModal",
    "globalSdl_FlashWindow",
    "globalSdl_SetWindowSize",
    "globalSdl_GetWindowSizeInPixels",
    "globalSdl_GetWindowFlags",
    "globalSdl_GetWindowDisplayScale",
    "globalSdl_SetWindowPosition",
    "globalSdl_SetWindowBordered",
    "globalSdl_SetWindowResizable",
    "globalSdl_SetWindowFullscreen",
    "globalSdl_SyncWindow",
    "globalSdl_CreatePopupWindow",
    "globalSdl_InitSubSystem",
    "globalSdl_QuitSubSystem",
    "globalSdl_AddGamepadMappingsFromFile",
    "globalSdl_SetJoystickEventsEnabled",
    "globalSdl_SetGamepadEventsEnabled",
    "globalSdl_DisableScreenSaver",
    "globalSdl_EnableScreenSaver",
    "globalSdl_CreateSystemCursor",
    "globalSdl_CreateColorCursor",
    "globalSdl_DestroyCursor",
    "globalSdl_SetCursor",
    "globalSdl_ShowCursor",
    "globalSdl_HideCursor",
    "globalSdl_OpenGamepad",
    "globalSdl_GetGamepadFromID",
    "globalSdl_CloseGamepad",
    "globalSdl_SetGamepadSensorEnabled",
    "globalSdl_GetGamepadName",
    "globalSdl_GetRealGamepadType",
    "globalSdl_SetGamepadLED",
    "globalSdl_RumbleGamepad",
    "globalSdl_RumbleGamepadTriggers",
    "globalSdl_GamepadHasSensor",
    "globalSdl_GetGamepadSensorData",
    "globalSdl_PollEvent",
    "globalSdl_AddEventWatch",
    "globalSdl_RemoveEventWatch",
    "globalSdl_GetWindowFromEvent",
    "globalSdl_SetWindowHitTest",
    "globalSdl_GetMouseState",
    "globalSdl_GetGlobalMouseState",
    "globalSdl_CaptureMouse",
    "globalSdl_GetMouseFocus",
    "globalSdl_GetKeyboardFocus",
    "globalSdl_SetWindowRelativeMouseMode",
    "globalSdl_WarpMouseInWindow",
    "globalSdl_GetModState",
    "globalSdl_GetKeyFromScancode",
    "globalSdl_GetKeyName",
    "globalSdl_SetModState",
    "globalSdl_GetWindowRelativeMouseMode",
    "globalSdl_StopTextInput",
    "globalSdl_ClearComposition",
    "globalSdl_SetTextInputArea",
    "globalSdl_StartTextInput",
    "globalSdl_CreateSurfaceFrom",
    "globalSdl_DestroySurface",
    "globalSdl_SetWindowIcon",
    "globalSdl_WindowHasSurface",
    "globalSdl_DestroyWindowSurface",
    "globalSdl_FillWindowColor",
    "globalSdl_PresentWindowPixels",
    "globalSdl_ShowOpenFolderDialogAt",
    "globalSdl_ShowOpenFileDialog",
    "globalSdl_ShowSaveFileDialog",
    "globalSdl_SetWindowExternalDropTarget",
    "globalSdl_BeginExternalDrag",
    "globalSdl_SetWindowMouseTransparent",
    "globalSdl_SetWindowCanMaximize",
    "globalSdl_SetWindowHasShadow",
    "globalSdl_SetWindowRoundedCorners",
    "globalSdl_SendWindowToBack",
    "globalSdl_RaiseWindowWithoutFocus",
    "globalSdl_SetWindowInTaskbar",
    "globalSdl_SetWindowCanClose",
    "globalSdl_SetWindowTaskbarOverlayIcon",
    "globalSteam_SteamHTMLSurface",
    "globalSteam_SteamAPI_RunCallbacks",
    "globalSteam_SteamGameServer_RunCallbacks",
    "globalSteam_SteamUser",
    "globalSteam_SteamRemoteStorage",
    "globalSteam_SteamFriends",
    "globalSteam_SteamNetworkingMessages",
    "globalSteam_SteamNetworkingUtils",
    "globalSteam_SteamMatchmaking",
    "globalSteam_SteamGameServer",
    "globalSteam_SteamApps",
    "globalSteam_SteamUtils",
    "globalSteam_SteamGameServer_BSecure",
    "globalSteam_SteamGameServer_GetSteamID",
    "globalSteam_SteamGameServer_Shutdown",
    "globalSteam_SteamGameServer_ReleaseCurrentThreadMemory",
    "globalSteam_SteamNetworkingSockets",
    "globalSteam_SteamGameServer_Init",
];

pub fn table() -> [usize; 3578] { [
    debug_error as *const () as usize,
    s1 as *const () as usize,
    s2 as *const () as usize,
    s3 as *const () as usize,
    s4 as *const () as usize,
    s5 as *const () as usize,
    s6 as *const () as usize,
    s7 as *const () as usize,
    s8 as *const () as usize,
    s9 as *const () as usize,
    s10 as *const () as usize,
    s11 as *const () as usize,
    s12 as *const () as usize,
    s13 as *const () as usize,
    s14 as *const () as usize,
    s15 as *const () as usize,
    s16 as *const () as usize,
    s17 as *const () as usize,
    s18 as *const () as usize,
    s19 as *const () as usize,
    s20 as *const () as usize,
    s21 as *const () as usize,
    s22 as *const () as usize,
    s23 as *const () as usize,
    s24 as *const () as usize,
    s25 as *const () as usize,
    s26 as *const () as usize,
    s27 as *const () as usize,
    s28 as *const () as usize,
    s29 as *const () as usize,
    s30 as *const () as usize,
    s31 as *const () as usize,
    s32 as *const () as usize,
    s33 as *const () as usize,
    s34 as *const () as usize,
    s35 as *const () as usize,
    s36 as *const () as usize,
    s37 as *const () as usize,
    s38 as *const () as usize,
    s39 as *const () as usize,
    s40 as *const () as usize,
    s41 as *const () as usize,
    s42 as *const () as usize,
    s43 as *const () as usize,
    s44 as *const () as usize,
    s45 as *const () as usize,
    s46 as *const () as usize,
    s47 as *const () as usize,
    s48 as *const () as usize,
    s49 as *const () as usize,
    s50 as *const () as usize,
    s51 as *const () as usize,
    s52 as *const () as usize,
    s53 as *const () as usize,
    s54 as *const () as usize,
    s55 as *const () as usize,
    s56 as *const () as usize,
    s57 as *const () as usize,
    s58 as *const () as usize,
    s59 as *const () as usize,
    s60 as *const () as usize,
    s61 as *const () as usize,
    s62 as *const () as usize,
    s63 as *const () as usize,
    s64 as *const () as usize,
    s65 as *const () as usize,
    s66 as *const () as usize,
    s67 as *const () as usize,
    s68 as *const () as usize,
    s69 as *const () as usize,
    s70 as *const () as usize,
    s71 as *const () as usize,
    s72 as *const () as usize,
    s73 as *const () as usize,
    s74 as *const () as usize,
    s75 as *const () as usize,
    s76 as *const () as usize,
    s77 as *const () as usize,
    s78 as *const () as usize,
    s79 as *const () as usize,
    s80 as *const () as usize,
    s81 as *const () as usize,
    s82 as *const () as usize,
    s83 as *const () as usize,
    s84 as *const () as usize,
    s85 as *const () as usize,
    s86 as *const () as usize,
    s87 as *const () as usize,
    s88 as *const () as usize,
    s89 as *const () as usize,
    s90 as *const () as usize,
    s91 as *const () as usize,
    s92 as *const () as usize,
    s93 as *const () as usize,
    s94 as *const () as usize,
    s95 as *const () as usize,
    s96 as *const () as usize,
    s97 as *const () as usize,
    s98 as *const () as usize,
    s99 as *const () as usize,
    s100 as *const () as usize,
    s101 as *const () as usize,
    s102 as *const () as usize,
    s103 as *const () as usize,
    s104 as *const () as usize,
    s105 as *const () as usize,
    s106 as *const () as usize,
    s107 as *const () as usize,
    s108 as *const () as usize,
    s109 as *const () as usize,
    s110 as *const () as usize,
    s111 as *const () as usize,
    s112 as *const () as usize,
    s113 as *const () as usize,
    s114 as *const () as usize,
    s115 as *const () as usize,
    s116 as *const () as usize,
    s117 as *const () as usize,
    s118 as *const () as usize,
    s119 as *const () as usize,
    s120 as *const () as usize,
    s121 as *const () as usize,
    s122 as *const () as usize,
    s123 as *const () as usize,
    s124 as *const () as usize,
    s125 as *const () as usize,
    s126 as *const () as usize,
    s127 as *const () as usize,
    s128 as *const () as usize,
    s129 as *const () as usize,
    s130 as *const () as usize,
    s131 as *const () as usize,
    s132 as *const () as usize,
    s133 as *const () as usize,
    s134 as *const () as usize,
    s135 as *const () as usize,
    s136 as *const () as usize,
    s137 as *const () as usize,
    s138 as *const () as usize,
    s139 as *const () as usize,
    s140 as *const () as usize,
    s141 as *const () as usize,
    s142 as *const () as usize,
    s143 as *const () as usize,
    s144 as *const () as usize,
    s145 as *const () as usize,
    s146 as *const () as usize,
    s147 as *const () as usize,
    s148 as *const () as usize,
    s149 as *const () as usize,
    s150 as *const () as usize,
    s151 as *const () as usize,
    s152 as *const () as usize,
    s153 as *const () as usize,
    s154 as *const () as usize,
    s155 as *const () as usize,
    s156 as *const () as usize,
    s157 as *const () as usize,
    s158 as *const () as usize,
    s159 as *const () as usize,
    s160 as *const () as usize,
    s161 as *const () as usize,
    s162 as *const () as usize,
    s163 as *const () as usize,
    s164 as *const () as usize,
    s165 as *const () as usize,
    s166 as *const () as usize,
    s167 as *const () as usize,
    s168 as *const () as usize,
    s169 as *const () as usize,
    s170 as *const () as usize,
    s171 as *const () as usize,
    s172 as *const () as usize,
    s173 as *const () as usize,
    s174 as *const () as usize,
    s175 as *const () as usize,
    s176 as *const () as usize,
    s177 as *const () as usize,
    s178 as *const () as usize,
    s179 as *const () as usize,
    s180 as *const () as usize,
    s181 as *const () as usize,
    s182 as *const () as usize,
    s183 as *const () as usize,
    s184 as *const () as usize,
    s185 as *const () as usize,
    s186 as *const () as usize,
    s187 as *const () as usize,
    s188 as *const () as usize,
    s189 as *const () as usize,
    s190 as *const () as usize,
    s191 as *const () as usize,
    s192 as *const () as usize,
    s193 as *const () as usize,
    s194 as *const () as usize,
    s195 as *const () as usize,
    s196 as *const () as usize,
    s197 as *const () as usize,
    s198 as *const () as usize,
    s199 as *const () as usize,
    s200 as *const () as usize,
    s201 as *const () as usize,
    s202 as *const () as usize,
    s203 as *const () as usize,
    s204 as *const () as usize,
    s205 as *const () as usize,
    s206 as *const () as usize,
    s207 as *const () as usize,
    s208 as *const () as usize,
    s209 as *const () as usize,
    s210 as *const () as usize,
    s211 as *const () as usize,
    s212 as *const () as usize,
    s213 as *const () as usize,
    s214 as *const () as usize,
    s215 as *const () as usize,
    s216 as *const () as usize,
    s217 as *const () as usize,
    s218 as *const () as usize,
    s219 as *const () as usize,
    s220 as *const () as usize,
    s221 as *const () as usize,
    s222 as *const () as usize,
    s223 as *const () as usize,
    s224 as *const () as usize,
    s225 as *const () as usize,
    s226 as *const () as usize,
    s227 as *const () as usize,
    s228 as *const () as usize,
    s229 as *const () as usize,
    s230 as *const () as usize,
    s231 as *const () as usize,
    s232 as *const () as usize,
    s233 as *const () as usize,
    s234 as *const () as usize,
    s235 as *const () as usize,
    s236 as *const () as usize,
    s237 as *const () as usize,
    s238 as *const () as usize,
    s239 as *const () as usize,
    s240 as *const () as usize,
    s241 as *const () as usize,
    s242 as *const () as usize,
    s243 as *const () as usize,
    s244 as *const () as usize,
    s245 as *const () as usize,
    s246 as *const () as usize,
    s247 as *const () as usize,
    s248 as *const () as usize,
    s249 as *const () as usize,
    s250 as *const () as usize,
    s251 as *const () as usize,
    s252 as *const () as usize,
    s253 as *const () as usize,
    s254 as *const () as usize,
    s255 as *const () as usize,
    s256 as *const () as usize,
    s257 as *const () as usize,
    s258 as *const () as usize,
    s259 as *const () as usize,
    s260 as *const () as usize,
    s261 as *const () as usize,
    s262 as *const () as usize,
    s263 as *const () as usize,
    s264 as *const () as usize,
    s265 as *const () as usize,
    s266 as *const () as usize,
    s267 as *const () as usize,
    s268 as *const () as usize,
    s269 as *const () as usize,
    s270 as *const () as usize,
    s271 as *const () as usize,
    s272 as *const () as usize,
    s273 as *const () as usize,
    s274 as *const () as usize,
    s275 as *const () as usize,
    s276 as *const () as usize,
    s277 as *const () as usize,
    s278 as *const () as usize,
    s279 as *const () as usize,
    s280 as *const () as usize,
    s281 as *const () as usize,
    s282 as *const () as usize,
    s283 as *const () as usize,
    s284 as *const () as usize,
    s285 as *const () as usize,
    s286 as *const () as usize,
    s287 as *const () as usize,
    s288 as *const () as usize,
    s289 as *const () as usize,
    s290 as *const () as usize,
    s291 as *const () as usize,
    s292 as *const () as usize,
    s293 as *const () as usize,
    s294 as *const () as usize,
    s295 as *const () as usize,
    s296 as *const () as usize,
    s297 as *const () as usize,
    s298 as *const () as usize,
    s299 as *const () as usize,
    s300 as *const () as usize,
    s301 as *const () as usize,
    s302 as *const () as usize,
    s303 as *const () as usize,
    s304 as *const () as usize,
    s305 as *const () as usize,
    s306 as *const () as usize,
    s307 as *const () as usize,
    s308 as *const () as usize,
    s309 as *const () as usize,
    s310 as *const () as usize,
    s311 as *const () as usize,
    s312 as *const () as usize,
    s313 as *const () as usize,
    s314 as *const () as usize,
    s315 as *const () as usize,
    s316 as *const () as usize,
    s317 as *const () as usize,
    s318 as *const () as usize,
    s319 as *const () as usize,
    s320 as *const () as usize,
    s321 as *const () as usize,
    s322 as *const () as usize,
    s323 as *const () as usize,
    s324 as *const () as usize,
    s325 as *const () as usize,
    s326 as *const () as usize,
    s327 as *const () as usize,
    s328 as *const () as usize,
    s329 as *const () as usize,
    s330 as *const () as usize,
    s331 as *const () as usize,
    s332 as *const () as usize,
    s333 as *const () as usize,
    s334 as *const () as usize,
    s335 as *const () as usize,
    s336 as *const () as usize,
    s337 as *const () as usize,
    s338 as *const () as usize,
    s339 as *const () as usize,
    s340 as *const () as usize,
    s341 as *const () as usize,
    s342 as *const () as usize,
    s343 as *const () as usize,
    s344 as *const () as usize,
    s345 as *const () as usize,
    s346 as *const () as usize,
    s347 as *const () as usize,
    s348 as *const () as usize,
    s349 as *const () as usize,
    s350 as *const () as usize,
    s351 as *const () as usize,
    s352 as *const () as usize,
    s353 as *const () as usize,
    s354 as *const () as usize,
    s355 as *const () as usize,
    s356 as *const () as usize,
    s357 as *const () as usize,
    s358 as *const () as usize,
    s359 as *const () as usize,
    s360 as *const () as usize,
    s361 as *const () as usize,
    s362 as *const () as usize,
    s363 as *const () as usize,
    s364 as *const () as usize,
    s365 as *const () as usize,
    s366 as *const () as usize,
    s367 as *const () as usize,
    s368 as *const () as usize,
    s369 as *const () as usize,
    s370 as *const () as usize,
    s371 as *const () as usize,
    s372 as *const () as usize,
    s373 as *const () as usize,
    s374 as *const () as usize,
    s375 as *const () as usize,
    s376 as *const () as usize,
    s377 as *const () as usize,
    s378 as *const () as usize,
    s379 as *const () as usize,
    s380 as *const () as usize,
    s381 as *const () as usize,
    s382 as *const () as usize,
    s383 as *const () as usize,
    s384 as *const () as usize,
    s385 as *const () as usize,
    s386 as *const () as usize,
    s387 as *const () as usize,
    s388 as *const () as usize,
    s389 as *const () as usize,
    s390 as *const () as usize,
    s391 as *const () as usize,
    s392 as *const () as usize,
    s393 as *const () as usize,
    s394 as *const () as usize,
    s395 as *const () as usize,
    s396 as *const () as usize,
    s397 as *const () as usize,
    s398 as *const () as usize,
    s399 as *const () as usize,
    s400 as *const () as usize,
    s401 as *const () as usize,
    s402 as *const () as usize,
    s403 as *const () as usize,
    s404 as *const () as usize,
    s405 as *const () as usize,
    s406 as *const () as usize,
    s407 as *const () as usize,
    s408 as *const () as usize,
    s409 as *const () as usize,
    s410 as *const () as usize,
    s411 as *const () as usize,
    s412 as *const () as usize,
    s413 as *const () as usize,
    s414 as *const () as usize,
    s415 as *const () as usize,
    s416 as *const () as usize,
    s417 as *const () as usize,
    s418 as *const () as usize,
    s419 as *const () as usize,
    s420 as *const () as usize,
    s421 as *const () as usize,
    s422 as *const () as usize,
    s423 as *const () as usize,
    s424 as *const () as usize,
    s425 as *const () as usize,
    s426 as *const () as usize,
    s427 as *const () as usize,
    s428 as *const () as usize,
    s429 as *const () as usize,
    s430 as *const () as usize,
    s431 as *const () as usize,
    s432 as *const () as usize,
    s433 as *const () as usize,
    s434 as *const () as usize,
    s435 as *const () as usize,
    s436 as *const () as usize,
    s437 as *const () as usize,
    s438 as *const () as usize,
    s439 as *const () as usize,
    s440 as *const () as usize,
    s441 as *const () as usize,
    s442 as *const () as usize,
    s443 as *const () as usize,
    s444 as *const () as usize,
    s445 as *const () as usize,
    s446 as *const () as usize,
    s447 as *const () as usize,
    s448 as *const () as usize,
    s449 as *const () as usize,
    s450 as *const () as usize,
    s451 as *const () as usize,
    s452 as *const () as usize,
    s453 as *const () as usize,
    s454 as *const () as usize,
    s455 as *const () as usize,
    s456 as *const () as usize,
    s457 as *const () as usize,
    s458 as *const () as usize,
    s459 as *const () as usize,
    s460 as *const () as usize,
    s461 as *const () as usize,
    s462 as *const () as usize,
    s463 as *const () as usize,
    s464 as *const () as usize,
    s465 as *const () as usize,
    s466 as *const () as usize,
    s467 as *const () as usize,
    s468 as *const () as usize,
    s469 as *const () as usize,
    s470 as *const () as usize,
    s471 as *const () as usize,
    s472 as *const () as usize,
    s473 as *const () as usize,
    s474 as *const () as usize,
    s475 as *const () as usize,
    s476 as *const () as usize,
    s477 as *const () as usize,
    s478 as *const () as usize,
    s479 as *const () as usize,
    s480 as *const () as usize,
    s481 as *const () as usize,
    s482 as *const () as usize,
    s483 as *const () as usize,
    s484 as *const () as usize,
    s485 as *const () as usize,
    s486 as *const () as usize,
    s487 as *const () as usize,
    s488 as *const () as usize,
    s489 as *const () as usize,
    s490 as *const () as usize,
    s491 as *const () as usize,
    s492 as *const () as usize,
    s493 as *const () as usize,
    s494 as *const () as usize,
    s495 as *const () as usize,
    s496 as *const () as usize,
    s497 as *const () as usize,
    s498 as *const () as usize,
    s499 as *const () as usize,
    s500 as *const () as usize,
    s501 as *const () as usize,
    s502 as *const () as usize,
    s503 as *const () as usize,
    s504 as *const () as usize,
    s505 as *const () as usize,
    s506 as *const () as usize,
    s507 as *const () as usize,
    s508 as *const () as usize,
    s509 as *const () as usize,
    s510 as *const () as usize,
    s511 as *const () as usize,
    s512 as *const () as usize,
    s513 as *const () as usize,
    s514 as *const () as usize,
    s515 as *const () as usize,
    s516 as *const () as usize,
    s517 as *const () as usize,
    s518 as *const () as usize,
    s519 as *const () as usize,
    s520 as *const () as usize,
    s521 as *const () as usize,
    s522 as *const () as usize,
    s523 as *const () as usize,
    s524 as *const () as usize,
    s525 as *const () as usize,
    s526 as *const () as usize,
    s527 as *const () as usize,
    s528 as *const () as usize,
    s529 as *const () as usize,
    s530 as *const () as usize,
    s531 as *const () as usize,
    s532 as *const () as usize,
    s533 as *const () as usize,
    s534 as *const () as usize,
    s535 as *const () as usize,
    s536 as *const () as usize,
    s537 as *const () as usize,
    s538 as *const () as usize,
    s539 as *const () as usize,
    s540 as *const () as usize,
    s541 as *const () as usize,
    s542 as *const () as usize,
    s543 as *const () as usize,
    s544 as *const () as usize,
    s545 as *const () as usize,
    s546 as *const () as usize,
    s547 as *const () as usize,
    s548 as *const () as usize,
    s549 as *const () as usize,
    s550 as *const () as usize,
    s551 as *const () as usize,
    s552 as *const () as usize,
    s553 as *const () as usize,
    s554 as *const () as usize,
    s555 as *const () as usize,
    s556 as *const () as usize,
    s557 as *const () as usize,
    s558 as *const () as usize,
    s559 as *const () as usize,
    s560 as *const () as usize,
    s561 as *const () as usize,
    s562 as *const () as usize,
    s563 as *const () as usize,
    s564 as *const () as usize,
    s565 as *const () as usize,
    s566 as *const () as usize,
    s567 as *const () as usize,
    s568 as *const () as usize,
    s569 as *const () as usize,
    s570 as *const () as usize,
    s571 as *const () as usize,
    s572 as *const () as usize,
    s573 as *const () as usize,
    s574 as *const () as usize,
    s575 as *const () as usize,
    s576 as *const () as usize,
    s577 as *const () as usize,
    s578 as *const () as usize,
    s579 as *const () as usize,
    s580 as *const () as usize,
    s581 as *const () as usize,
    s582 as *const () as usize,
    s583 as *const () as usize,
    s584 as *const () as usize,
    s585 as *const () as usize,
    s586 as *const () as usize,
    s587 as *const () as usize,
    s588 as *const () as usize,
    s589 as *const () as usize,
    s590 as *const () as usize,
    s591 as *const () as usize,
    s592 as *const () as usize,
    s593 as *const () as usize,
    s594 as *const () as usize,
    s595 as *const () as usize,
    s596 as *const () as usize,
    s597 as *const () as usize,
    s598 as *const () as usize,
    s599 as *const () as usize,
    s600 as *const () as usize,
    s601 as *const () as usize,
    s602 as *const () as usize,
    s603 as *const () as usize,
    s604 as *const () as usize,
    s605 as *const () as usize,
    s606 as *const () as usize,
    s607 as *const () as usize,
    s608 as *const () as usize,
    s609 as *const () as usize,
    s610 as *const () as usize,
    s611 as *const () as usize,
    s612 as *const () as usize,
    s613 as *const () as usize,
    s614 as *const () as usize,
    s615 as *const () as usize,
    s616 as *const () as usize,
    s617 as *const () as usize,
    s618 as *const () as usize,
    s619 as *const () as usize,
    s620 as *const () as usize,
    s621 as *const () as usize,
    s622 as *const () as usize,
    s623 as *const () as usize,
    s624 as *const () as usize,
    s625 as *const () as usize,
    s626 as *const () as usize,
    s627 as *const () as usize,
    s628 as *const () as usize,
    s629 as *const () as usize,
    s630 as *const () as usize,
    s631 as *const () as usize,
    s632 as *const () as usize,
    s633 as *const () as usize,
    s634 as *const () as usize,
    s635 as *const () as usize,
    s636 as *const () as usize,
    s637 as *const () as usize,
    s638 as *const () as usize,
    s639 as *const () as usize,
    s640 as *const () as usize,
    s641 as *const () as usize,
    s642 as *const () as usize,
    s643 as *const () as usize,
    s644 as *const () as usize,
    s645 as *const () as usize,
    s646 as *const () as usize,
    s647 as *const () as usize,
    s648 as *const () as usize,
    s649 as *const () as usize,
    s650 as *const () as usize,
    s651 as *const () as usize,
    s652 as *const () as usize,
    s653 as *const () as usize,
    s654 as *const () as usize,
    s655 as *const () as usize,
    s656 as *const () as usize,
    s657 as *const () as usize,
    s658 as *const () as usize,
    s659 as *const () as usize,
    s660 as *const () as usize,
    s661 as *const () as usize,
    s662 as *const () as usize,
    s663 as *const () as usize,
    s664 as *const () as usize,
    s665 as *const () as usize,
    s666 as *const () as usize,
    s667 as *const () as usize,
    s668 as *const () as usize,
    s669 as *const () as usize,
    s670 as *const () as usize,
    s671 as *const () as usize,
    s672 as *const () as usize,
    s673 as *const () as usize,
    s674 as *const () as usize,
    s675 as *const () as usize,
    s676 as *const () as usize,
    s677 as *const () as usize,
    s678 as *const () as usize,
    s679 as *const () as usize,
    s680 as *const () as usize,
    s681 as *const () as usize,
    s682 as *const () as usize,
    s683 as *const () as usize,
    s684 as *const () as usize,
    s685 as *const () as usize,
    s686 as *const () as usize,
    s687 as *const () as usize,
    s688 as *const () as usize,
    s689 as *const () as usize,
    s690 as *const () as usize,
    s691 as *const () as usize,
    s692 as *const () as usize,
    s693 as *const () as usize,
    s694 as *const () as usize,
    s695 as *const () as usize,
    s696 as *const () as usize,
    s697 as *const () as usize,
    s698 as *const () as usize,
    s699 as *const () as usize,
    s700 as *const () as usize,
    s701 as *const () as usize,
    s702 as *const () as usize,
    s703 as *const () as usize,
    s704 as *const () as usize,
    s705 as *const () as usize,
    s706 as *const () as usize,
    s707 as *const () as usize,
    s708 as *const () as usize,
    s709 as *const () as usize,
    s710 as *const () as usize,
    s711 as *const () as usize,
    s712 as *const () as usize,
    s713 as *const () as usize,
    s714 as *const () as usize,
    s715 as *const () as usize,
    s716 as *const () as usize,
    s717 as *const () as usize,
    s718 as *const () as usize,
    s719 as *const () as usize,
    s720 as *const () as usize,
    s721 as *const () as usize,
    s722 as *const () as usize,
    s723 as *const () as usize,
    s724 as *const () as usize,
    s725 as *const () as usize,
    s726 as *const () as usize,
    s727 as *const () as usize,
    s728 as *const () as usize,
    s729 as *const () as usize,
    s730 as *const () as usize,
    s731 as *const () as usize,
    s732 as *const () as usize,
    s733 as *const () as usize,
    s734 as *const () as usize,
    s735 as *const () as usize,
    s736 as *const () as usize,
    s737 as *const () as usize,
    s738 as *const () as usize,
    s739 as *const () as usize,
    s740 as *const () as usize,
    s741 as *const () as usize,
    s742 as *const () as usize,
    s743 as *const () as usize,
    s744 as *const () as usize,
    s745 as *const () as usize,
    s746 as *const () as usize,
    s747 as *const () as usize,
    s748 as *const () as usize,
    s749 as *const () as usize,
    s750 as *const () as usize,
    s751 as *const () as usize,
    s752 as *const () as usize,
    s753 as *const () as usize,
    s754 as *const () as usize,
    s755 as *const () as usize,
    s756 as *const () as usize,
    s757 as *const () as usize,
    s758 as *const () as usize,
    s759 as *const () as usize,
    s760 as *const () as usize,
    s761 as *const () as usize,
    s762 as *const () as usize,
    s763 as *const () as usize,
    s764 as *const () as usize,
    s765 as *const () as usize,
    s766 as *const () as usize,
    s767 as *const () as usize,
    s768 as *const () as usize,
    s769 as *const () as usize,
    s770 as *const () as usize,
    s771 as *const () as usize,
    s772 as *const () as usize,
    s773 as *const () as usize,
    s774 as *const () as usize,
    s775 as *const () as usize,
    s776 as *const () as usize,
    s777 as *const () as usize,
    s778 as *const () as usize,
    s779 as *const () as usize,
    s780 as *const () as usize,
    s781 as *const () as usize,
    s782 as *const () as usize,
    s783 as *const () as usize,
    s784 as *const () as usize,
    s785 as *const () as usize,
    s786 as *const () as usize,
    s787 as *const () as usize,
    s788 as *const () as usize,
    s789 as *const () as usize,
    s790 as *const () as usize,
    s791 as *const () as usize,
    s792 as *const () as usize,
    s793 as *const () as usize,
    s794 as *const () as usize,
    s795 as *const () as usize,
    s796 as *const () as usize,
    s797 as *const () as usize,
    s798 as *const () as usize,
    s799 as *const () as usize,
    s800 as *const () as usize,
    s801 as *const () as usize,
    s802 as *const () as usize,
    s803 as *const () as usize,
    s804 as *const () as usize,
    s805 as *const () as usize,
    s806 as *const () as usize,
    s807 as *const () as usize,
    s808 as *const () as usize,
    s809 as *const () as usize,
    s810 as *const () as usize,
    s811 as *const () as usize,
    s812 as *const () as usize,
    s813 as *const () as usize,
    s814 as *const () as usize,
    s815 as *const () as usize,
    s816 as *const () as usize,
    s817 as *const () as usize,
    s818 as *const () as usize,
    s819 as *const () as usize,
    s820 as *const () as usize,
    s821 as *const () as usize,
    s822 as *const () as usize,
    s823 as *const () as usize,
    s824 as *const () as usize,
    s825 as *const () as usize,
    s826 as *const () as usize,
    s827 as *const () as usize,
    s828 as *const () as usize,
    s829 as *const () as usize,
    s830 as *const () as usize,
    s831 as *const () as usize,
    s832 as *const () as usize,
    s833 as *const () as usize,
    s834 as *const () as usize,
    s835 as *const () as usize,
    s836 as *const () as usize,
    s837 as *const () as usize,
    s838 as *const () as usize,
    s839 as *const () as usize,
    s840 as *const () as usize,
    s841 as *const () as usize,
    s842 as *const () as usize,
    s843 as *const () as usize,
    s844 as *const () as usize,
    s845 as *const () as usize,
    s846 as *const () as usize,
    s847 as *const () as usize,
    s848 as *const () as usize,
    s849 as *const () as usize,
    s850 as *const () as usize,
    s851 as *const () as usize,
    s852 as *const () as usize,
    s853 as *const () as usize,
    s854 as *const () as usize,
    s855 as *const () as usize,
    s856 as *const () as usize,
    s857 as *const () as usize,
    s858 as *const () as usize,
    s859 as *const () as usize,
    s860 as *const () as usize,
    s861 as *const () as usize,
    s862 as *const () as usize,
    s863 as *const () as usize,
    s864 as *const () as usize,
    s865 as *const () as usize,
    s866 as *const () as usize,
    s867 as *const () as usize,
    s868 as *const () as usize,
    s869 as *const () as usize,
    s870 as *const () as usize,
    s871 as *const () as usize,
    s872 as *const () as usize,
    s873 as *const () as usize,
    s874 as *const () as usize,
    s875 as *const () as usize,
    s876 as *const () as usize,
    s877 as *const () as usize,
    s878 as *const () as usize,
    s879 as *const () as usize,
    s880 as *const () as usize,
    s881 as *const () as usize,
    s882 as *const () as usize,
    s883 as *const () as usize,
    s884 as *const () as usize,
    s885 as *const () as usize,
    s886 as *const () as usize,
    s887 as *const () as usize,
    s888 as *const () as usize,
    s889 as *const () as usize,
    s890 as *const () as usize,
    s891 as *const () as usize,
    s892 as *const () as usize,
    s893 as *const () as usize,
    s894 as *const () as usize,
    s895 as *const () as usize,
    s896 as *const () as usize,
    s897 as *const () as usize,
    s898 as *const () as usize,
    s899 as *const () as usize,
    s900 as *const () as usize,
    s901 as *const () as usize,
    s902 as *const () as usize,
    s903 as *const () as usize,
    s904 as *const () as usize,
    s905 as *const () as usize,
    s906 as *const () as usize,
    s907 as *const () as usize,
    s908 as *const () as usize,
    s909 as *const () as usize,
    s910 as *const () as usize,
    s911 as *const () as usize,
    s912 as *const () as usize,
    s913 as *const () as usize,
    s914 as *const () as usize,
    s915 as *const () as usize,
    s916 as *const () as usize,
    s917 as *const () as usize,
    s918 as *const () as usize,
    s919 as *const () as usize,
    s920 as *const () as usize,
    s921 as *const () as usize,
    s922 as *const () as usize,
    s923 as *const () as usize,
    s924 as *const () as usize,
    s925 as *const () as usize,
    s926 as *const () as usize,
    s927 as *const () as usize,
    s928 as *const () as usize,
    s929 as *const () as usize,
    s930 as *const () as usize,
    s931 as *const () as usize,
    s932 as *const () as usize,
    s933 as *const () as usize,
    s934 as *const () as usize,
    s935 as *const () as usize,
    s936 as *const () as usize,
    s937 as *const () as usize,
    s938 as *const () as usize,
    s939 as *const () as usize,
    s940 as *const () as usize,
    s941 as *const () as usize,
    s942 as *const () as usize,
    s943 as *const () as usize,
    s944 as *const () as usize,
    s945 as *const () as usize,
    s946 as *const () as usize,
    s947 as *const () as usize,
    s948 as *const () as usize,
    s949 as *const () as usize,
    s950 as *const () as usize,
    s951 as *const () as usize,
    s952 as *const () as usize,
    s953 as *const () as usize,
    s954 as *const () as usize,
    s955 as *const () as usize,
    s956 as *const () as usize,
    s957 as *const () as usize,
    s958 as *const () as usize,
    s959 as *const () as usize,
    s960 as *const () as usize,
    s961 as *const () as usize,
    s962 as *const () as usize,
    s963 as *const () as usize,
    s964 as *const () as usize,
    s965 as *const () as usize,
    s966 as *const () as usize,
    s967 as *const () as usize,
    s968 as *const () as usize,
    s969 as *const () as usize,
    s970 as *const () as usize,
    s971 as *const () as usize,
    s972 as *const () as usize,
    s973 as *const () as usize,
    s974 as *const () as usize,
    s975 as *const () as usize,
    s976 as *const () as usize,
    s977 as *const () as usize,
    s978 as *const () as usize,
    s979 as *const () as usize,
    s980 as *const () as usize,
    s981 as *const () as usize,
    s982 as *const () as usize,
    s983 as *const () as usize,
    s984 as *const () as usize,
    s985 as *const () as usize,
    s986 as *const () as usize,
    s987 as *const () as usize,
    s988 as *const () as usize,
    s989 as *const () as usize,
    s990 as *const () as usize,
    s991 as *const () as usize,
    s992 as *const () as usize,
    s993 as *const () as usize,
    s994 as *const () as usize,
    s995 as *const () as usize,
    s996 as *const () as usize,
    s997 as *const () as usize,
    s998 as *const () as usize,
    s999 as *const () as usize,
    s1000 as *const () as usize,
    s1001 as *const () as usize,
    s1002 as *const () as usize,
    s1003 as *const () as usize,
    s1004 as *const () as usize,
    s1005 as *const () as usize,
    s1006 as *const () as usize,
    s1007 as *const () as usize,
    s1008 as *const () as usize,
    s1009 as *const () as usize,
    s1010 as *const () as usize,
    s1011 as *const () as usize,
    s1012 as *const () as usize,
    s1013 as *const () as usize,
    s1014 as *const () as usize,
    s1015 as *const () as usize,
    s1016 as *const () as usize,
    s1017 as *const () as usize,
    s1018 as *const () as usize,
    s1019 as *const () as usize,
    s1020 as *const () as usize,
    s1021 as *const () as usize,
    s1022 as *const () as usize,
    s1023 as *const () as usize,
    s1024 as *const () as usize,
    s1025 as *const () as usize,
    s1026 as *const () as usize,
    s1027 as *const () as usize,
    s1028 as *const () as usize,
    s1029 as *const () as usize,
    s1030 as *const () as usize,
    s1031 as *const () as usize,
    s1032 as *const () as usize,
    s1033 as *const () as usize,
    s1034 as *const () as usize,
    s1035 as *const () as usize,
    s1036 as *const () as usize,
    s1037 as *const () as usize,
    s1038 as *const () as usize,
    s1039 as *const () as usize,
    s1040 as *const () as usize,
    s1041 as *const () as usize,
    s1042 as *const () as usize,
    s1043 as *const () as usize,
    s1044 as *const () as usize,
    s1045 as *const () as usize,
    s1046 as *const () as usize,
    s1047 as *const () as usize,
    s1048 as *const () as usize,
    s1049 as *const () as usize,
    s1050 as *const () as usize,
    s1051 as *const () as usize,
    s1052 as *const () as usize,
    s1053 as *const () as usize,
    s1054 as *const () as usize,
    s1055 as *const () as usize,
    s1056 as *const () as usize,
    s1057 as *const () as usize,
    s1058 as *const () as usize,
    s1059 as *const () as usize,
    s1060 as *const () as usize,
    s1061 as *const () as usize,
    s1062 as *const () as usize,
    s1063 as *const () as usize,
    s1064 as *const () as usize,
    s1065 as *const () as usize,
    s1066 as *const () as usize,
    s1067 as *const () as usize,
    s1068 as *const () as usize,
    s1069 as *const () as usize,
    s1070 as *const () as usize,
    s1071 as *const () as usize,
    s1072 as *const () as usize,
    s1073 as *const () as usize,
    s1074 as *const () as usize,
    s1075 as *const () as usize,
    s1076 as *const () as usize,
    s1077 as *const () as usize,
    s1078 as *const () as usize,
    s1079 as *const () as usize,
    s1080 as *const () as usize,
    s1081 as *const () as usize,
    s1082 as *const () as usize,
    s1083 as *const () as usize,
    s1084 as *const () as usize,
    s1085 as *const () as usize,
    s1086 as *const () as usize,
    s1087 as *const () as usize,
    s1088 as *const () as usize,
    s1089 as *const () as usize,
    s1090 as *const () as usize,
    s1091 as *const () as usize,
    s1092 as *const () as usize,
    s1093 as *const () as usize,
    s1094 as *const () as usize,
    s1095 as *const () as usize,
    s1096 as *const () as usize,
    s1097 as *const () as usize,
    s1098 as *const () as usize,
    s1099 as *const () as usize,
    s1100 as *const () as usize,
    s1101 as *const () as usize,
    s1102 as *const () as usize,
    s1103 as *const () as usize,
    s1104 as *const () as usize,
    s1105 as *const () as usize,
    s1106 as *const () as usize,
    s1107 as *const () as usize,
    s1108 as *const () as usize,
    s1109 as *const () as usize,
    s1110 as *const () as usize,
    s1111 as *const () as usize,
    s1112 as *const () as usize,
    s1113 as *const () as usize,
    s1114 as *const () as usize,
    s1115 as *const () as usize,
    s1116 as *const () as usize,
    s1117 as *const () as usize,
    s1118 as *const () as usize,
    s1119 as *const () as usize,
    s1120 as *const () as usize,
    s1121 as *const () as usize,
    s1122 as *const () as usize,
    s1123 as *const () as usize,
    s1124 as *const () as usize,
    s1125 as *const () as usize,
    s1126 as *const () as usize,
    s1127 as *const () as usize,
    s1128 as *const () as usize,
    s1129 as *const () as usize,
    s1130 as *const () as usize,
    s1131 as *const () as usize,
    s1132 as *const () as usize,
    s1133 as *const () as usize,
    s1134 as *const () as usize,
    s1135 as *const () as usize,
    s1136 as *const () as usize,
    s1137 as *const () as usize,
    s1138 as *const () as usize,
    s1139 as *const () as usize,
    s1140 as *const () as usize,
    s1141 as *const () as usize,
    s1142 as *const () as usize,
    s1143 as *const () as usize,
    s1144 as *const () as usize,
    s1145 as *const () as usize,
    s1146 as *const () as usize,
    s1147 as *const () as usize,
    s1148 as *const () as usize,
    s1149 as *const () as usize,
    s1150 as *const () as usize,
    s1151 as *const () as usize,
    s1152 as *const () as usize,
    s1153 as *const () as usize,
    s1154 as *const () as usize,
    s1155 as *const () as usize,
    s1156 as *const () as usize,
    s1157 as *const () as usize,
    s1158 as *const () as usize,
    s1159 as *const () as usize,
    s1160 as *const () as usize,
    s1161 as *const () as usize,
    s1162 as *const () as usize,
    s1163 as *const () as usize,
    s1164 as *const () as usize,
    s1165 as *const () as usize,
    s1166 as *const () as usize,
    s1167 as *const () as usize,
    s1168 as *const () as usize,
    s1169 as *const () as usize,
    s1170 as *const () as usize,
    s1171 as *const () as usize,
    s1172 as *const () as usize,
    s1173 as *const () as usize,
    s1174 as *const () as usize,
    s1175 as *const () as usize,
    s1176 as *const () as usize,
    s1177 as *const () as usize,
    s1178 as *const () as usize,
    s1179 as *const () as usize,
    s1180 as *const () as usize,
    s1181 as *const () as usize,
    s1182 as *const () as usize,
    s1183 as *const () as usize,
    s1184 as *const () as usize,
    s1185 as *const () as usize,
    s1186 as *const () as usize,
    s1187 as *const () as usize,
    s1188 as *const () as usize,
    s1189 as *const () as usize,
    s1190 as *const () as usize,
    s1191 as *const () as usize,
    s1192 as *const () as usize,
    s1193 as *const () as usize,
    s1194 as *const () as usize,
    s1195 as *const () as usize,
    s1196 as *const () as usize,
    s1197 as *const () as usize,
    s1198 as *const () as usize,
    s1199 as *const () as usize,
    s1200 as *const () as usize,
    s1201 as *const () as usize,
    s1202 as *const () as usize,
    s1203 as *const () as usize,
    s1204 as *const () as usize,
    s1205 as *const () as usize,
    s1206 as *const () as usize,
    s1207 as *const () as usize,
    s1208 as *const () as usize,
    s1209 as *const () as usize,
    s1210 as *const () as usize,
    s1211 as *const () as usize,
    s1212 as *const () as usize,
    s1213 as *const () as usize,
    s1214 as *const () as usize,
    s1215 as *const () as usize,
    s1216 as *const () as usize,
    s1217 as *const () as usize,
    s1218 as *const () as usize,
    s1219 as *const () as usize,
    s1220 as *const () as usize,
    s1221 as *const () as usize,
    s1222 as *const () as usize,
    s1223 as *const () as usize,
    s1224 as *const () as usize,
    s1225 as *const () as usize,
    s1226 as *const () as usize,
    s1227 as *const () as usize,
    s1228 as *const () as usize,
    s1229 as *const () as usize,
    s1230 as *const () as usize,
    s1231 as *const () as usize,
    s1232 as *const () as usize,
    s1233 as *const () as usize,
    s1234 as *const () as usize,
    s1235 as *const () as usize,
    s1236 as *const () as usize,
    s1237 as *const () as usize,
    s1238 as *const () as usize,
    s1239 as *const () as usize,
    s1240 as *const () as usize,
    s1241 as *const () as usize,
    s1242 as *const () as usize,
    s1243 as *const () as usize,
    s1244 as *const () as usize,
    s1245 as *const () as usize,
    s1246 as *const () as usize,
    s1247 as *const () as usize,
    s1248 as *const () as usize,
    s1249 as *const () as usize,
    s1250 as *const () as usize,
    s1251 as *const () as usize,
    s1252 as *const () as usize,
    s1253 as *const () as usize,
    s1254 as *const () as usize,
    s1255 as *const () as usize,
    s1256 as *const () as usize,
    s1257 as *const () as usize,
    s1258 as *const () as usize,
    s1259 as *const () as usize,
    s1260 as *const () as usize,
    s1261 as *const () as usize,
    s1262 as *const () as usize,
    s1263 as *const () as usize,
    s1264 as *const () as usize,
    s1265 as *const () as usize,
    s1266 as *const () as usize,
    s1267 as *const () as usize,
    s1268 as *const () as usize,
    s1269 as *const () as usize,
    s1270 as *const () as usize,
    s1271 as *const () as usize,
    s1272 as *const () as usize,
    s1273 as *const () as usize,
    s1274 as *const () as usize,
    s1275 as *const () as usize,
    s1276 as *const () as usize,
    s1277 as *const () as usize,
    s1278 as *const () as usize,
    s1279 as *const () as usize,
    s1280 as *const () as usize,
    s1281 as *const () as usize,
    s1282 as *const () as usize,
    s1283 as *const () as usize,
    s1284 as *const () as usize,
    s1285 as *const () as usize,
    s1286 as *const () as usize,
    s1287 as *const () as usize,
    s1288 as *const () as usize,
    s1289 as *const () as usize,
    s1290 as *const () as usize,
    s1291 as *const () as usize,
    s1292 as *const () as usize,
    s1293 as *const () as usize,
    s1294 as *const () as usize,
    s1295 as *const () as usize,
    s1296 as *const () as usize,
    s1297 as *const () as usize,
    s1298 as *const () as usize,
    s1299 as *const () as usize,
    s1300 as *const () as usize,
    s1301 as *const () as usize,
    s1302 as *const () as usize,
    s1303 as *const () as usize,
    s1304 as *const () as usize,
    s1305 as *const () as usize,
    s1306 as *const () as usize,
    s1307 as *const () as usize,
    s1308 as *const () as usize,
    s1309 as *const () as usize,
    s1310 as *const () as usize,
    s1311 as *const () as usize,
    s1312 as *const () as usize,
    s1313 as *const () as usize,
    s1314 as *const () as usize,
    s1315 as *const () as usize,
    s1316 as *const () as usize,
    s1317 as *const () as usize,
    s1318 as *const () as usize,
    s1319 as *const () as usize,
    s1320 as *const () as usize,
    s1321 as *const () as usize,
    s1322 as *const () as usize,
    s1323 as *const () as usize,
    s1324 as *const () as usize,
    s1325 as *const () as usize,
    s1326 as *const () as usize,
    s1327 as *const () as usize,
    s1328 as *const () as usize,
    s1329 as *const () as usize,
    s1330 as *const () as usize,
    s1331 as *const () as usize,
    s1332 as *const () as usize,
    s1333 as *const () as usize,
    s1334 as *const () as usize,
    s1335 as *const () as usize,
    s1336 as *const () as usize,
    s1337 as *const () as usize,
    s1338 as *const () as usize,
    s1339 as *const () as usize,
    s1340 as *const () as usize,
    s1341 as *const () as usize,
    s1342 as *const () as usize,
    s1343 as *const () as usize,
    s1344 as *const () as usize,
    s1345 as *const () as usize,
    s1346 as *const () as usize,
    s1347 as *const () as usize,
    s1348 as *const () as usize,
    s1349 as *const () as usize,
    s1350 as *const () as usize,
    s1351 as *const () as usize,
    s1352 as *const () as usize,
    s1353 as *const () as usize,
    s1354 as *const () as usize,
    s1355 as *const () as usize,
    s1356 as *const () as usize,
    s1357 as *const () as usize,
    s1358 as *const () as usize,
    s1359 as *const () as usize,
    s1360 as *const () as usize,
    s1361 as *const () as usize,
    s1362 as *const () as usize,
    s1363 as *const () as usize,
    s1364 as *const () as usize,
    s1365 as *const () as usize,
    s1366 as *const () as usize,
    s1367 as *const () as usize,
    s1368 as *const () as usize,
    s1369 as *const () as usize,
    s1370 as *const () as usize,
    s1371 as *const () as usize,
    s1372 as *const () as usize,
    s1373 as *const () as usize,
    s1374 as *const () as usize,
    s1375 as *const () as usize,
    s1376 as *const () as usize,
    s1377 as *const () as usize,
    s1378 as *const () as usize,
    s1379 as *const () as usize,
    s1380 as *const () as usize,
    s1381 as *const () as usize,
    s1382 as *const () as usize,
    s1383 as *const () as usize,
    s1384 as *const () as usize,
    s1385 as *const () as usize,
    s1386 as *const () as usize,
    s1387 as *const () as usize,
    s1388 as *const () as usize,
    s1389 as *const () as usize,
    s1390 as *const () as usize,
    s1391 as *const () as usize,
    s1392 as *const () as usize,
    s1393 as *const () as usize,
    s1394 as *const () as usize,
    s1395 as *const () as usize,
    s1396 as *const () as usize,
    s1397 as *const () as usize,
    s1398 as *const () as usize,
    s1399 as *const () as usize,
    s1400 as *const () as usize,
    s1401 as *const () as usize,
    s1402 as *const () as usize,
    s1403 as *const () as usize,
    s1404 as *const () as usize,
    s1405 as *const () as usize,
    s1406 as *const () as usize,
    s1407 as *const () as usize,
    s1408 as *const () as usize,
    s1409 as *const () as usize,
    s1410 as *const () as usize,
    s1411 as *const () as usize,
    s1412 as *const () as usize,
    s1413 as *const () as usize,
    s1414 as *const () as usize,
    s1415 as *const () as usize,
    s1416 as *const () as usize,
    s1417 as *const () as usize,
    s1418 as *const () as usize,
    s1419 as *const () as usize,
    s1420 as *const () as usize,
    s1421 as *const () as usize,
    s1422 as *const () as usize,
    s1423 as *const () as usize,
    s1424 as *const () as usize,
    s1425 as *const () as usize,
    s1426 as *const () as usize,
    s1427 as *const () as usize,
    s1428 as *const () as usize,
    s1429 as *const () as usize,
    s1430 as *const () as usize,
    s1431 as *const () as usize,
    s1432 as *const () as usize,
    s1433 as *const () as usize,
    s1434 as *const () as usize,
    s1435 as *const () as usize,
    s1436 as *const () as usize,
    s1437 as *const () as usize,
    s1438 as *const () as usize,
    s1439 as *const () as usize,
    s1440 as *const () as usize,
    s1441 as *const () as usize,
    s1442 as *const () as usize,
    s1443 as *const () as usize,
    s1444 as *const () as usize,
    s1445 as *const () as usize,
    s1446 as *const () as usize,
    s1447 as *const () as usize,
    s1448 as *const () as usize,
    s1449 as *const () as usize,
    s1450 as *const () as usize,
    s1451 as *const () as usize,
    s1452 as *const () as usize,
    s1453 as *const () as usize,
    s1454 as *const () as usize,
    s1455 as *const () as usize,
    s1456 as *const () as usize,
    s1457 as *const () as usize,
    s1458 as *const () as usize,
    s1459 as *const () as usize,
    s1460 as *const () as usize,
    s1461 as *const () as usize,
    s1462 as *const () as usize,
    s1463 as *const () as usize,
    s1464 as *const () as usize,
    s1465 as *const () as usize,
    s1466 as *const () as usize,
    s1467 as *const () as usize,
    s1468 as *const () as usize,
    s1469 as *const () as usize,
    s1470 as *const () as usize,
    s1471 as *const () as usize,
    s1472 as *const () as usize,
    s1473 as *const () as usize,
    s1474 as *const () as usize,
    s1475 as *const () as usize,
    s1476 as *const () as usize,
    s1477 as *const () as usize,
    s1478 as *const () as usize,
    s1479 as *const () as usize,
    s1480 as *const () as usize,
    s1481 as *const () as usize,
    s1482 as *const () as usize,
    s1483 as *const () as usize,
    s1484 as *const () as usize,
    s1485 as *const () as usize,
    s1486 as *const () as usize,
    s1487 as *const () as usize,
    s1488 as *const () as usize,
    s1489 as *const () as usize,
    s1490 as *const () as usize,
    s1491 as *const () as usize,
    s1492 as *const () as usize,
    s1493 as *const () as usize,
    s1494 as *const () as usize,
    s1495 as *const () as usize,
    s1496 as *const () as usize,
    s1497 as *const () as usize,
    s1498 as *const () as usize,
    s1499 as *const () as usize,
    s1500 as *const () as usize,
    s1501 as *const () as usize,
    s1502 as *const () as usize,
    s1503 as *const () as usize,
    s1504 as *const () as usize,
    s1505 as *const () as usize,
    s1506 as *const () as usize,
    s1507 as *const () as usize,
    s1508 as *const () as usize,
    s1509 as *const () as usize,
    s1510 as *const () as usize,
    s1511 as *const () as usize,
    s1512 as *const () as usize,
    s1513 as *const () as usize,
    s1514 as *const () as usize,
    s1515 as *const () as usize,
    s1516 as *const () as usize,
    s1517 as *const () as usize,
    s1518 as *const () as usize,
    s1519 as *const () as usize,
    s1520 as *const () as usize,
    s1521 as *const () as usize,
    s1522 as *const () as usize,
    s1523 as *const () as usize,
    s1524 as *const () as usize,
    s1525 as *const () as usize,
    s1526 as *const () as usize,
    s1527 as *const () as usize,
    s1528 as *const () as usize,
    s1529 as *const () as usize,
    s1530 as *const () as usize,
    s1531 as *const () as usize,
    s1532 as *const () as usize,
    s1533 as *const () as usize,
    s1534 as *const () as usize,
    s1535 as *const () as usize,
    s1536 as *const () as usize,
    s1537 as *const () as usize,
    s1538 as *const () as usize,
    s1539 as *const () as usize,
    s1540 as *const () as usize,
    s1541 as *const () as usize,
    s1542 as *const () as usize,
    s1543 as *const () as usize,
    s1544 as *const () as usize,
    s1545 as *const () as usize,
    s1546 as *const () as usize,
    s1547 as *const () as usize,
    s1548 as *const () as usize,
    s1549 as *const () as usize,
    s1550 as *const () as usize,
    s1551 as *const () as usize,
    s1552 as *const () as usize,
    s1553 as *const () as usize,
    s1554 as *const () as usize,
    s1555 as *const () as usize,
    s1556 as *const () as usize,
    s1557 as *const () as usize,
    s1558 as *const () as usize,
    s1559 as *const () as usize,
    s1560 as *const () as usize,
    s1561 as *const () as usize,
    s1562 as *const () as usize,
    s1563 as *const () as usize,
    s1564 as *const () as usize,
    s1565 as *const () as usize,
    s1566 as *const () as usize,
    s1567 as *const () as usize,
    s1568 as *const () as usize,
    s1569 as *const () as usize,
    s1570 as *const () as usize,
    s1571 as *const () as usize,
    s1572 as *const () as usize,
    s1573 as *const () as usize,
    s1574 as *const () as usize,
    s1575 as *const () as usize,
    s1576 as *const () as usize,
    s1577 as *const () as usize,
    s1578 as *const () as usize,
    s1579 as *const () as usize,
    s1580 as *const () as usize,
    s1581 as *const () as usize,
    s1582 as *const () as usize,
    s1583 as *const () as usize,
    s1584 as *const () as usize,
    s1585 as *const () as usize,
    s1586 as *const () as usize,
    s1587 as *const () as usize,
    s1588 as *const () as usize,
    s1589 as *const () as usize,
    s1590 as *const () as usize,
    s1591 as *const () as usize,
    s1592 as *const () as usize,
    s1593 as *const () as usize,
    s1594 as *const () as usize,
    s1595 as *const () as usize,
    s1596 as *const () as usize,
    s1597 as *const () as usize,
    s1598 as *const () as usize,
    s1599 as *const () as usize,
    s1600 as *const () as usize,
    s1601 as *const () as usize,
    s1602 as *const () as usize,
    s1603 as *const () as usize,
    s1604 as *const () as usize,
    s1605 as *const () as usize,
    s1606 as *const () as usize,
    s1607 as *const () as usize,
    s1608 as *const () as usize,
    s1609 as *const () as usize,
    s1610 as *const () as usize,
    s1611 as *const () as usize,
    s1612 as *const () as usize,
    s1613 as *const () as usize,
    s1614 as *const () as usize,
    s1615 as *const () as usize,
    s1616 as *const () as usize,
    s1617 as *const () as usize,
    s1618 as *const () as usize,
    s1619 as *const () as usize,
    s1620 as *const () as usize,
    s1621 as *const () as usize,
    s1622 as *const () as usize,
    s1623 as *const () as usize,
    s1624 as *const () as usize,
    s1625 as *const () as usize,
    s1626 as *const () as usize,
    s1627 as *const () as usize,
    s1628 as *const () as usize,
    s1629 as *const () as usize,
    s1630 as *const () as usize,
    s1631 as *const () as usize,
    s1632 as *const () as usize,
    s1633 as *const () as usize,
    s1634 as *const () as usize,
    s1635 as *const () as usize,
    s1636 as *const () as usize,
    s1637 as *const () as usize,
    s1638 as *const () as usize,
    s1639 as *const () as usize,
    s1640 as *const () as usize,
    s1641 as *const () as usize,
    s1642 as *const () as usize,
    s1643 as *const () as usize,
    s1644 as *const () as usize,
    s1645 as *const () as usize,
    s1646 as *const () as usize,
    s1647 as *const () as usize,
    s1648 as *const () as usize,
    s1649 as *const () as usize,
    s1650 as *const () as usize,
    s1651 as *const () as usize,
    s1652 as *const () as usize,
    s1653 as *const () as usize,
    s1654 as *const () as usize,
    s1655 as *const () as usize,
    s1656 as *const () as usize,
    s1657 as *const () as usize,
    s1658 as *const () as usize,
    s1659 as *const () as usize,
    s1660 as *const () as usize,
    s1661 as *const () as usize,
    s1662 as *const () as usize,
    s1663 as *const () as usize,
    s1664 as *const () as usize,
    s1665 as *const () as usize,
    s1666 as *const () as usize,
    s1667 as *const () as usize,
    s1668 as *const () as usize,
    s1669 as *const () as usize,
    s1670 as *const () as usize,
    s1671 as *const () as usize,
    s1672 as *const () as usize,
    s1673 as *const () as usize,
    s1674 as *const () as usize,
    s1675 as *const () as usize,
    s1676 as *const () as usize,
    s1677 as *const () as usize,
    s1678 as *const () as usize,
    s1679 as *const () as usize,
    s1680 as *const () as usize,
    s1681 as *const () as usize,
    s1682 as *const () as usize,
    s1683 as *const () as usize,
    s1684 as *const () as usize,
    s1685 as *const () as usize,
    s1686 as *const () as usize,
    s1687 as *const () as usize,
    s1688 as *const () as usize,
    s1689 as *const () as usize,
    s1690 as *const () as usize,
    s1691 as *const () as usize,
    s1692 as *const () as usize,
    s1693 as *const () as usize,
    s1694 as *const () as usize,
    s1695 as *const () as usize,
    s1696 as *const () as usize,
    s1697 as *const () as usize,
    s1698 as *const () as usize,
    s1699 as *const () as usize,
    s1700 as *const () as usize,
    s1701 as *const () as usize,
    s1702 as *const () as usize,
    s1703 as *const () as usize,
    s1704 as *const () as usize,
    s1705 as *const () as usize,
    s1706 as *const () as usize,
    s1707 as *const () as usize,
    s1708 as *const () as usize,
    s1709 as *const () as usize,
    s1710 as *const () as usize,
    s1711 as *const () as usize,
    s1712 as *const () as usize,
    s1713 as *const () as usize,
    s1714 as *const () as usize,
    s1715 as *const () as usize,
    s1716 as *const () as usize,
    s1717 as *const () as usize,
    s1718 as *const () as usize,
    s1719 as *const () as usize,
    s1720 as *const () as usize,
    s1721 as *const () as usize,
    s1722 as *const () as usize,
    s1723 as *const () as usize,
    s1724 as *const () as usize,
    s1725 as *const () as usize,
    s1726 as *const () as usize,
    s1727 as *const () as usize,
    s1728 as *const () as usize,
    s1729 as *const () as usize,
    s1730 as *const () as usize,
    s1731 as *const () as usize,
    s1732 as *const () as usize,
    s1733 as *const () as usize,
    s1734 as *const () as usize,
    s1735 as *const () as usize,
    s1736 as *const () as usize,
    s1737 as *const () as usize,
    s1738 as *const () as usize,
    s1739 as *const () as usize,
    s1740 as *const () as usize,
    s1741 as *const () as usize,
    s1742 as *const () as usize,
    s1743 as *const () as usize,
    s1744 as *const () as usize,
    s1745 as *const () as usize,
    s1746 as *const () as usize,
    s1747 as *const () as usize,
    s1748 as *const () as usize,
    s1749 as *const () as usize,
    s1750 as *const () as usize,
    s1751 as *const () as usize,
    s1752 as *const () as usize,
    s1753 as *const () as usize,
    s1754 as *const () as usize,
    s1755 as *const () as usize,
    s1756 as *const () as usize,
    s1757 as *const () as usize,
    s1758 as *const () as usize,
    s1759 as *const () as usize,
    s1760 as *const () as usize,
    s1761 as *const () as usize,
    s1762 as *const () as usize,
    s1763 as *const () as usize,
    s1764 as *const () as usize,
    s1765 as *const () as usize,
    s1766 as *const () as usize,
    s1767 as *const () as usize,
    s1768 as *const () as usize,
    s1769 as *const () as usize,
    s1770 as *const () as usize,
    s1771 as *const () as usize,
    s1772 as *const () as usize,
    s1773 as *const () as usize,
    s1774 as *const () as usize,
    s1775 as *const () as usize,
    s1776 as *const () as usize,
    s1777 as *const () as usize,
    s1778 as *const () as usize,
    s1779 as *const () as usize,
    s1780 as *const () as usize,
    s1781 as *const () as usize,
    s1782 as *const () as usize,
    s1783 as *const () as usize,
    s1784 as *const () as usize,
    s1785 as *const () as usize,
    s1786 as *const () as usize,
    s1787 as *const () as usize,
    s1788 as *const () as usize,
    s1789 as *const () as usize,
    s1790 as *const () as usize,
    s1791 as *const () as usize,
    s1792 as *const () as usize,
    s1793 as *const () as usize,
    s1794 as *const () as usize,
    s1795 as *const () as usize,
    s1796 as *const () as usize,
    s1797 as *const () as usize,
    s1798 as *const () as usize,
    s1799 as *const () as usize,
    s1800 as *const () as usize,
    s1801 as *const () as usize,
    s1802 as *const () as usize,
    s1803 as *const () as usize,
    s1804 as *const () as usize,
    s1805 as *const () as usize,
    s1806 as *const () as usize,
    s1807 as *const () as usize,
    s1808 as *const () as usize,
    s1809 as *const () as usize,
    s1810 as *const () as usize,
    s1811 as *const () as usize,
    s1812 as *const () as usize,
    s1813 as *const () as usize,
    s1814 as *const () as usize,
    s1815 as *const () as usize,
    s1816 as *const () as usize,
    s1817 as *const () as usize,
    s1818 as *const () as usize,
    s1819 as *const () as usize,
    s1820 as *const () as usize,
    s1821 as *const () as usize,
    s1822 as *const () as usize,
    s1823 as *const () as usize,
    s1824 as *const () as usize,
    s1825 as *const () as usize,
    s1826 as *const () as usize,
    s1827 as *const () as usize,
    s1828 as *const () as usize,
    s1829 as *const () as usize,
    s1830 as *const () as usize,
    s1831 as *const () as usize,
    s1832 as *const () as usize,
    s1833 as *const () as usize,
    s1834 as *const () as usize,
    s1835 as *const () as usize,
    s1836 as *const () as usize,
    s1837 as *const () as usize,
    s1838 as *const () as usize,
    s1839 as *const () as usize,
    s1840 as *const () as usize,
    s1841 as *const () as usize,
    s1842 as *const () as usize,
    s1843 as *const () as usize,
    s1844 as *const () as usize,
    s1845 as *const () as usize,
    s1846 as *const () as usize,
    s1847 as *const () as usize,
    s1848 as *const () as usize,
    s1849 as *const () as usize,
    s1850 as *const () as usize,
    s1851 as *const () as usize,
    s1852 as *const () as usize,
    s1853 as *const () as usize,
    s1854 as *const () as usize,
    s1855 as *const () as usize,
    s1856 as *const () as usize,
    s1857 as *const () as usize,
    s1858 as *const () as usize,
    s1859 as *const () as usize,
    s1860 as *const () as usize,
    s1861 as *const () as usize,
    s1862 as *const () as usize,
    s1863 as *const () as usize,
    s1864 as *const () as usize,
    s1865 as *const () as usize,
    s1866 as *const () as usize,
    s1867 as *const () as usize,
    s1868 as *const () as usize,
    s1869 as *const () as usize,
    s1870 as *const () as usize,
    s1871 as *const () as usize,
    s1872 as *const () as usize,
    s1873 as *const () as usize,
    s1874 as *const () as usize,
    s1875 as *const () as usize,
    s1876 as *const () as usize,
    s1877 as *const () as usize,
    s1878 as *const () as usize,
    s1879 as *const () as usize,
    s1880 as *const () as usize,
    s1881 as *const () as usize,
    s1882 as *const () as usize,
    s1883 as *const () as usize,
    s1884 as *const () as usize,
    s1885 as *const () as usize,
    s1886 as *const () as usize,
    s1887 as *const () as usize,
    s1888 as *const () as usize,
    s1889 as *const () as usize,
    s1890 as *const () as usize,
    s1891 as *const () as usize,
    s1892 as *const () as usize,
    s1893 as *const () as usize,
    s1894 as *const () as usize,
    s1895 as *const () as usize,
    s1896 as *const () as usize,
    s1897 as *const () as usize,
    s1898 as *const () as usize,
    s1899 as *const () as usize,
    s1900 as *const () as usize,
    s1901 as *const () as usize,
    s1902 as *const () as usize,
    s1903 as *const () as usize,
    s1904 as *const () as usize,
    s1905 as *const () as usize,
    s1906 as *const () as usize,
    s1907 as *const () as usize,
    s1908 as *const () as usize,
    s1909 as *const () as usize,
    s1910 as *const () as usize,
    s1911 as *const () as usize,
    s1912 as *const () as usize,
    s1913 as *const () as usize,
    s1914 as *const () as usize,
    s1915 as *const () as usize,
    s1916 as *const () as usize,
    s1917 as *const () as usize,
    s1918 as *const () as usize,
    s1919 as *const () as usize,
    s1920 as *const () as usize,
    s1921 as *const () as usize,
    s1922 as *const () as usize,
    s1923 as *const () as usize,
    s1924 as *const () as usize,
    s1925 as *const () as usize,
    s1926 as *const () as usize,
    s1927 as *const () as usize,
    s1928 as *const () as usize,
    s1929 as *const () as usize,
    s1930 as *const () as usize,
    s1931 as *const () as usize,
    s1932 as *const () as usize,
    s1933 as *const () as usize,
    s1934 as *const () as usize,
    s1935 as *const () as usize,
    s1936 as *const () as usize,
    s1937 as *const () as usize,
    s1938 as *const () as usize,
    s1939 as *const () as usize,
    s1940 as *const () as usize,
    s1941 as *const () as usize,
    s1942 as *const () as usize,
    s1943 as *const () as usize,
    s1944 as *const () as usize,
    s1945 as *const () as usize,
    s1946 as *const () as usize,
    s1947 as *const () as usize,
    s1948 as *const () as usize,
    s1949 as *const () as usize,
    s1950 as *const () as usize,
    s1951 as *const () as usize,
    s1952 as *const () as usize,
    s1953 as *const () as usize,
    s1954 as *const () as usize,
    s1955 as *const () as usize,
    s1956 as *const () as usize,
    s1957 as *const () as usize,
    s1958 as *const () as usize,
    s1959 as *const () as usize,
    s1960 as *const () as usize,
    s1961 as *const () as usize,
    s1962 as *const () as usize,
    s1963 as *const () as usize,
    s1964 as *const () as usize,
    s1965 as *const () as usize,
    s1966 as *const () as usize,
    s1967 as *const () as usize,
    s1968 as *const () as usize,
    s1969 as *const () as usize,
    s1970 as *const () as usize,
    s1971 as *const () as usize,
    s1972 as *const () as usize,
    s1973 as *const () as usize,
    s1974 as *const () as usize,
    s1975 as *const () as usize,
    s1976 as *const () as usize,
    s1977 as *const () as usize,
    s1978 as *const () as usize,
    s1979 as *const () as usize,
    s1980 as *const () as usize,
    s1981 as *const () as usize,
    s1982 as *const () as usize,
    s1983 as *const () as usize,
    s1984 as *const () as usize,
    s1985 as *const () as usize,
    s1986 as *const () as usize,
    s1987 as *const () as usize,
    s1988 as *const () as usize,
    s1989 as *const () as usize,
    s1990 as *const () as usize,
    s1991 as *const () as usize,
    s1992 as *const () as usize,
    s1993 as *const () as usize,
    s1994 as *const () as usize,
    s1995 as *const () as usize,
    s1996 as *const () as usize,
    s1997 as *const () as usize,
    s1998 as *const () as usize,
    s1999 as *const () as usize,
    s2000 as *const () as usize,
    s2001 as *const () as usize,
    s2002 as *const () as usize,
    s2003 as *const () as usize,
    s2004 as *const () as usize,
    s2005 as *const () as usize,
    s2006 as *const () as usize,
    s2007 as *const () as usize,
    s2008 as *const () as usize,
    s2009 as *const () as usize,
    s2010 as *const () as usize,
    s2011 as *const () as usize,
    s2012 as *const () as usize,
    s2013 as *const () as usize,
    s2014 as *const () as usize,
    s2015 as *const () as usize,
    s2016 as *const () as usize,
    s2017 as *const () as usize,
    s2018 as *const () as usize,
    s2019 as *const () as usize,
    s2020 as *const () as usize,
    s2021 as *const () as usize,
    s2022 as *const () as usize,
    s2023 as *const () as usize,
    s2024 as *const () as usize,
    s2025 as *const () as usize,
    s2026 as *const () as usize,
    s2027 as *const () as usize,
    s2028 as *const () as usize,
    s2029 as *const () as usize,
    s2030 as *const () as usize,
    s2031 as *const () as usize,
    s2032 as *const () as usize,
    s2033 as *const () as usize,
    s2034 as *const () as usize,
    s2035 as *const () as usize,
    s2036 as *const () as usize,
    s2037 as *const () as usize,
    s2038 as *const () as usize,
    s2039 as *const () as usize,
    s2040 as *const () as usize,
    s2041 as *const () as usize,
    s2042 as *const () as usize,
    s2043 as *const () as usize,
    s2044 as *const () as usize,
    s2045 as *const () as usize,
    s2046 as *const () as usize,
    s2047 as *const () as usize,
    s2048 as *const () as usize,
    s2049 as *const () as usize,
    s2050 as *const () as usize,
    s2051 as *const () as usize,
    s2052 as *const () as usize,
    s2053 as *const () as usize,
    s2054 as *const () as usize,
    s2055 as *const () as usize,
    s2056 as *const () as usize,
    s2057 as *const () as usize,
    s2058 as *const () as usize,
    s2059 as *const () as usize,
    s2060 as *const () as usize,
    s2061 as *const () as usize,
    s2062 as *const () as usize,
    s2063 as *const () as usize,
    s2064 as *const () as usize,
    s2065 as *const () as usize,
    s2066 as *const () as usize,
    s2067 as *const () as usize,
    s2068 as *const () as usize,
    s2069 as *const () as usize,
    s2070 as *const () as usize,
    s2071 as *const () as usize,
    s2072 as *const () as usize,
    s2073 as *const () as usize,
    s2074 as *const () as usize,
    s2075 as *const () as usize,
    s2076 as *const () as usize,
    s2077 as *const () as usize,
    s2078 as *const () as usize,
    s2079 as *const () as usize,
    s2080 as *const () as usize,
    s2081 as *const () as usize,
    s2082 as *const () as usize,
    s2083 as *const () as usize,
    s2084 as *const () as usize,
    s2085 as *const () as usize,
    s2086 as *const () as usize,
    s2087 as *const () as usize,
    s2088 as *const () as usize,
    s2089 as *const () as usize,
    s2090 as *const () as usize,
    s2091 as *const () as usize,
    s2092 as *const () as usize,
    s2093 as *const () as usize,
    s2094 as *const () as usize,
    s2095 as *const () as usize,
    s2096 as *const () as usize,
    s2097 as *const () as usize,
    s2098 as *const () as usize,
    s2099 as *const () as usize,
    s2100 as *const () as usize,
    s2101 as *const () as usize,
    s2102 as *const () as usize,
    s2103 as *const () as usize,
    s2104 as *const () as usize,
    s2105 as *const () as usize,
    s2106 as *const () as usize,
    s2107 as *const () as usize,
    s2108 as *const () as usize,
    s2109 as *const () as usize,
    s2110 as *const () as usize,
    s2111 as *const () as usize,
    s2112 as *const () as usize,
    s2113 as *const () as usize,
    s2114 as *const () as usize,
    s2115 as *const () as usize,
    s2116 as *const () as usize,
    s2117 as *const () as usize,
    s2118 as *const () as usize,
    s2119 as *const () as usize,
    s2120 as *const () as usize,
    s2121 as *const () as usize,
    s2122 as *const () as usize,
    s2123 as *const () as usize,
    s2124 as *const () as usize,
    s2125 as *const () as usize,
    s2126 as *const () as usize,
    s2127 as *const () as usize,
    s2128 as *const () as usize,
    s2129 as *const () as usize,
    s2130 as *const () as usize,
    s2131 as *const () as usize,
    s2132 as *const () as usize,
    s2133 as *const () as usize,
    s2134 as *const () as usize,
    s2135 as *const () as usize,
    s2136 as *const () as usize,
    s2137 as *const () as usize,
    s2138 as *const () as usize,
    s2139 as *const () as usize,
    s2140 as *const () as usize,
    s2141 as *const () as usize,
    s2142 as *const () as usize,
    s2143 as *const () as usize,
    s2144 as *const () as usize,
    s2145 as *const () as usize,
    s2146 as *const () as usize,
    s2147 as *const () as usize,
    s2148 as *const () as usize,
    s2149 as *const () as usize,
    s2150 as *const () as usize,
    s2151 as *const () as usize,
    s2152 as *const () as usize,
    s2153 as *const () as usize,
    s2154 as *const () as usize,
    s2155 as *const () as usize,
    s2156 as *const () as usize,
    s2157 as *const () as usize,
    s2158 as *const () as usize,
    s2159 as *const () as usize,
    s2160 as *const () as usize,
    s2161 as *const () as usize,
    s2162 as *const () as usize,
    s2163 as *const () as usize,
    s2164 as *const () as usize,
    s2165 as *const () as usize,
    s2166 as *const () as usize,
    s2167 as *const () as usize,
    s2168 as *const () as usize,
    s2169 as *const () as usize,
    s2170 as *const () as usize,
    s2171 as *const () as usize,
    s2172 as *const () as usize,
    s2173 as *const () as usize,
    s2174 as *const () as usize,
    s2175 as *const () as usize,
    s2176 as *const () as usize,
    s2177 as *const () as usize,
    s2178 as *const () as usize,
    s2179 as *const () as usize,
    s2180 as *const () as usize,
    s2181 as *const () as usize,
    s2182 as *const () as usize,
    s2183 as *const () as usize,
    s2184 as *const () as usize,
    s2185 as *const () as usize,
    s2186 as *const () as usize,
    s2187 as *const () as usize,
    s2188 as *const () as usize,
    s2189 as *const () as usize,
    s2190 as *const () as usize,
    s2191 as *const () as usize,
    s2192 as *const () as usize,
    s2193 as *const () as usize,
    s2194 as *const () as usize,
    s2195 as *const () as usize,
    s2196 as *const () as usize,
    s2197 as *const () as usize,
    s2198 as *const () as usize,
    s2199 as *const () as usize,
    s2200 as *const () as usize,
    s2201 as *const () as usize,
    s2202 as *const () as usize,
    s2203 as *const () as usize,
    s2204 as *const () as usize,
    s2205 as *const () as usize,
    s2206 as *const () as usize,
    s2207 as *const () as usize,
    s2208 as *const () as usize,
    s2209 as *const () as usize,
    s2210 as *const () as usize,
    s2211 as *const () as usize,
    s2212 as *const () as usize,
    s2213 as *const () as usize,
    s2214 as *const () as usize,
    s2215 as *const () as usize,
    s2216 as *const () as usize,
    s2217 as *const () as usize,
    s2218 as *const () as usize,
    s2219 as *const () as usize,
    s2220 as *const () as usize,
    s2221 as *const () as usize,
    s2222 as *const () as usize,
    s2223 as *const () as usize,
    s2224 as *const () as usize,
    s2225 as *const () as usize,
    s2226 as *const () as usize,
    s2227 as *const () as usize,
    s2228 as *const () as usize,
    s2229 as *const () as usize,
    s2230 as *const () as usize,
    s2231 as *const () as usize,
    s2232 as *const () as usize,
    s2233 as *const () as usize,
    s2234 as *const () as usize,
    s2235 as *const () as usize,
    s2236 as *const () as usize,
    s2237 as *const () as usize,
    s2238 as *const () as usize,
    s2239 as *const () as usize,
    s2240 as *const () as usize,
    s2241 as *const () as usize,
    s2242 as *const () as usize,
    s2243 as *const () as usize,
    s2244 as *const () as usize,
    s2245 as *const () as usize,
    s2246 as *const () as usize,
    s2247 as *const () as usize,
    s2248 as *const () as usize,
    s2249 as *const () as usize,
    s2250 as *const () as usize,
    s2251 as *const () as usize,
    s2252 as *const () as usize,
    s2253 as *const () as usize,
    s2254 as *const () as usize,
    s2255 as *const () as usize,
    s2256 as *const () as usize,
    s2257 as *const () as usize,
    s2258 as *const () as usize,
    s2259 as *const () as usize,
    s2260 as *const () as usize,
    s2261 as *const () as usize,
    s2262 as *const () as usize,
    s2263 as *const () as usize,
    s2264 as *const () as usize,
    s2265 as *const () as usize,
    s2266 as *const () as usize,
    s2267 as *const () as usize,
    s2268 as *const () as usize,
    s2269 as *const () as usize,
    s2270 as *const () as usize,
    s2271 as *const () as usize,
    s2272 as *const () as usize,
    s2273 as *const () as usize,
    s2274 as *const () as usize,
    s2275 as *const () as usize,
    s2276 as *const () as usize,
    s2277 as *const () as usize,
    s2278 as *const () as usize,
    s2279 as *const () as usize,
    s2280 as *const () as usize,
    s2281 as *const () as usize,
    s2282 as *const () as usize,
    s2283 as *const () as usize,
    s2284 as *const () as usize,
    s2285 as *const () as usize,
    s2286 as *const () as usize,
    s2287 as *const () as usize,
    s2288 as *const () as usize,
    s2289 as *const () as usize,
    s2290 as *const () as usize,
    s2291 as *const () as usize,
    s2292 as *const () as usize,
    s2293 as *const () as usize,
    s2294 as *const () as usize,
    s2295 as *const () as usize,
    s2296 as *const () as usize,
    s2297 as *const () as usize,
    s2298 as *const () as usize,
    s2299 as *const () as usize,
    s2300 as *const () as usize,
    s2301 as *const () as usize,
    s2302 as *const () as usize,
    s2303 as *const () as usize,
    s2304 as *const () as usize,
    s2305 as *const () as usize,
    s2306 as *const () as usize,
    s2307 as *const () as usize,
    s2308 as *const () as usize,
    s2309 as *const () as usize,
    s2310 as *const () as usize,
    s2311 as *const () as usize,
    s2312 as *const () as usize,
    s2313 as *const () as usize,
    s2314 as *const () as usize,
    s2315 as *const () as usize,
    s2316 as *const () as usize,
    s2317 as *const () as usize,
    s2318 as *const () as usize,
    s2319 as *const () as usize,
    s2320 as *const () as usize,
    s2321 as *const () as usize,
    s2322 as *const () as usize,
    s2323 as *const () as usize,
    s2324 as *const () as usize,
    s2325 as *const () as usize,
    s2326 as *const () as usize,
    s2327 as *const () as usize,
    s2328 as *const () as usize,
    s2329 as *const () as usize,
    s2330 as *const () as usize,
    s2331 as *const () as usize,
    s2332 as *const () as usize,
    s2333 as *const () as usize,
    s2334 as *const () as usize,
    s2335 as *const () as usize,
    s2336 as *const () as usize,
    s2337 as *const () as usize,
    s2338 as *const () as usize,
    s2339 as *const () as usize,
    s2340 as *const () as usize,
    s2341 as *const () as usize,
    s2342 as *const () as usize,
    s2343 as *const () as usize,
    s2344 as *const () as usize,
    s2345 as *const () as usize,
    s2346 as *const () as usize,
    s2347 as *const () as usize,
    s2348 as *const () as usize,
    s2349 as *const () as usize,
    s2350 as *const () as usize,
    s2351 as *const () as usize,
    s2352 as *const () as usize,
    s2353 as *const () as usize,
    s2354 as *const () as usize,
    s2355 as *const () as usize,
    s2356 as *const () as usize,
    s2357 as *const () as usize,
    s2358 as *const () as usize,
    s2359 as *const () as usize,
    s2360 as *const () as usize,
    s2361 as *const () as usize,
    s2362 as *const () as usize,
    s2363 as *const () as usize,
    s2364 as *const () as usize,
    s2365 as *const () as usize,
    s2366 as *const () as usize,
    s2367 as *const () as usize,
    s2368 as *const () as usize,
    s2369 as *const () as usize,
    s2370 as *const () as usize,
    s2371 as *const () as usize,
    s2372 as *const () as usize,
    s2373 as *const () as usize,
    s2374 as *const () as usize,
    s2375 as *const () as usize,
    s2376 as *const () as usize,
    s2377 as *const () as usize,
    s2378 as *const () as usize,
    s2379 as *const () as usize,
    s2380 as *const () as usize,
    s2381 as *const () as usize,
    s2382 as *const () as usize,
    s2383 as *const () as usize,
    s2384 as *const () as usize,
    s2385 as *const () as usize,
    s2386 as *const () as usize,
    s2387 as *const () as usize,
    s2388 as *const () as usize,
    s2389 as *const () as usize,
    s2390 as *const () as usize,
    s2391 as *const () as usize,
    s2392 as *const () as usize,
    s2393 as *const () as usize,
    s2394 as *const () as usize,
    s2395 as *const () as usize,
    s2396 as *const () as usize,
    s2397 as *const () as usize,
    s2398 as *const () as usize,
    s2399 as *const () as usize,
    s2400 as *const () as usize,
    s2401 as *const () as usize,
    s2402 as *const () as usize,
    s2403 as *const () as usize,
    s2404 as *const () as usize,
    s2405 as *const () as usize,
    s2406 as *const () as usize,
    s2407 as *const () as usize,
    s2408 as *const () as usize,
    s2409 as *const () as usize,
    s2410 as *const () as usize,
    s2411 as *const () as usize,
    s2412 as *const () as usize,
    s2413 as *const () as usize,
    s2414 as *const () as usize,
    s2415 as *const () as usize,
    s2416 as *const () as usize,
    s2417 as *const () as usize,
    s2418 as *const () as usize,
    s2419 as *const () as usize,
    s2420 as *const () as usize,
    s2421 as *const () as usize,
    s2422 as *const () as usize,
    s2423 as *const () as usize,
    s2424 as *const () as usize,
    s2425 as *const () as usize,
    s2426 as *const () as usize,
    s2427 as *const () as usize,
    s2428 as *const () as usize,
    s2429 as *const () as usize,
    s2430 as *const () as usize,
    s2431 as *const () as usize,
    s2432 as *const () as usize,
    s2433 as *const () as usize,
    s2434 as *const () as usize,
    s2435 as *const () as usize,
    s2436 as *const () as usize,
    s2437 as *const () as usize,
    s2438 as *const () as usize,
    s2439 as *const () as usize,
    s2440 as *const () as usize,
    s2441 as *const () as usize,
    s2442 as *const () as usize,
    s2443 as *const () as usize,
    s2444 as *const () as usize,
    s2445 as *const () as usize,
    s2446 as *const () as usize,
    s2447 as *const () as usize,
    s2448 as *const () as usize,
    s2449 as *const () as usize,
    s2450 as *const () as usize,
    s2451 as *const () as usize,
    s2452 as *const () as usize,
    s2453 as *const () as usize,
    s2454 as *const () as usize,
    s2455 as *const () as usize,
    s2456 as *const () as usize,
    s2457 as *const () as usize,
    s2458 as *const () as usize,
    s2459 as *const () as usize,
    s2460 as *const () as usize,
    s2461 as *const () as usize,
    s2462 as *const () as usize,
    s2463 as *const () as usize,
    s2464 as *const () as usize,
    s2465 as *const () as usize,
    s2466 as *const () as usize,
    s2467 as *const () as usize,
    s2468 as *const () as usize,
    s2469 as *const () as usize,
    s2470 as *const () as usize,
    s2471 as *const () as usize,
    s2472 as *const () as usize,
    s2473 as *const () as usize,
    s2474 as *const () as usize,
    s2475 as *const () as usize,
    s2476 as *const () as usize,
    s2477 as *const () as usize,
    s2478 as *const () as usize,
    s2479 as *const () as usize,
    s2480 as *const () as usize,
    s2481 as *const () as usize,
    s2482 as *const () as usize,
    s2483 as *const () as usize,
    s2484 as *const () as usize,
    s2485 as *const () as usize,
    s2486 as *const () as usize,
    s2487 as *const () as usize,
    s2488 as *const () as usize,
    s2489 as *const () as usize,
    s2490 as *const () as usize,
    s2491 as *const () as usize,
    s2492 as *const () as usize,
    s2493 as *const () as usize,
    s2494 as *const () as usize,
    s2495 as *const () as usize,
    s2496 as *const () as usize,
    s2497 as *const () as usize,
    s2498 as *const () as usize,
    s2499 as *const () as usize,
    s2500 as *const () as usize,
    s2501 as *const () as usize,
    s2502 as *const () as usize,
    s2503 as *const () as usize,
    s2504 as *const () as usize,
    s2505 as *const () as usize,
    s2506 as *const () as usize,
    s2507 as *const () as usize,
    s2508 as *const () as usize,
    s2509 as *const () as usize,
    s2510 as *const () as usize,
    s2511 as *const () as usize,
    s2512 as *const () as usize,
    s2513 as *const () as usize,
    s2514 as *const () as usize,
    s2515 as *const () as usize,
    s2516 as *const () as usize,
    s2517 as *const () as usize,
    s2518 as *const () as usize,
    s2519 as *const () as usize,
    s2520 as *const () as usize,
    s2521 as *const () as usize,
    s2522 as *const () as usize,
    s2523 as *const () as usize,
    s2524 as *const () as usize,
    s2525 as *const () as usize,
    s2526 as *const () as usize,
    s2527 as *const () as usize,
    s2528 as *const () as usize,
    s2529 as *const () as usize,
    s2530 as *const () as usize,
    s2531 as *const () as usize,
    s2532 as *const () as usize,
    s2533 as *const () as usize,
    s2534 as *const () as usize,
    s2535 as *const () as usize,
    s2536 as *const () as usize,
    s2537 as *const () as usize,
    s2538 as *const () as usize,
    s2539 as *const () as usize,
    s2540 as *const () as usize,
    s2541 as *const () as usize,
    s2542 as *const () as usize,
    s2543 as *const () as usize,
    s2544 as *const () as usize,
    s2545 as *const () as usize,
    s2546 as *const () as usize,
    s2547 as *const () as usize,
    s2548 as *const () as usize,
    s2549 as *const () as usize,
    s2550 as *const () as usize,
    s2551 as *const () as usize,
    s2552 as *const () as usize,
    s2553 as *const () as usize,
    s2554 as *const () as usize,
    s2555 as *const () as usize,
    s2556 as *const () as usize,
    s2557 as *const () as usize,
    s2558 as *const () as usize,
    s2559 as *const () as usize,
    s2560 as *const () as usize,
    s2561 as *const () as usize,
    s2562 as *const () as usize,
    s2563 as *const () as usize,
    s2564 as *const () as usize,
    s2565 as *const () as usize,
    s2566 as *const () as usize,
    s2567 as *const () as usize,
    s2568 as *const () as usize,
    s2569 as *const () as usize,
    s2570 as *const () as usize,
    s2571 as *const () as usize,
    s2572 as *const () as usize,
    s2573 as *const () as usize,
    s2574 as *const () as usize,
    s2575 as *const () as usize,
    s2576 as *const () as usize,
    s2577 as *const () as usize,
    s2578 as *const () as usize,
    s2579 as *const () as usize,
    s2580 as *const () as usize,
    s2581 as *const () as usize,
    s2582 as *const () as usize,
    s2583 as *const () as usize,
    s2584 as *const () as usize,
    s2585 as *const () as usize,
    s2586 as *const () as usize,
    s2587 as *const () as usize,
    s2588 as *const () as usize,
    s2589 as *const () as usize,
    s2590 as *const () as usize,
    s2591 as *const () as usize,
    s2592 as *const () as usize,
    s2593 as *const () as usize,
    s2594 as *const () as usize,
    s2595 as *const () as usize,
    s2596 as *const () as usize,
    s2597 as *const () as usize,
    s2598 as *const () as usize,
    s2599 as *const () as usize,
    s2600 as *const () as usize,
    s2601 as *const () as usize,
    s2602 as *const () as usize,
    s2603 as *const () as usize,
    s2604 as *const () as usize,
    s2605 as *const () as usize,
    s2606 as *const () as usize,
    s2607 as *const () as usize,
    s2608 as *const () as usize,
    s2609 as *const () as usize,
    s2610 as *const () as usize,
    s2611 as *const () as usize,
    s2612 as *const () as usize,
    s2613 as *const () as usize,
    s2614 as *const () as usize,
    s2615 as *const () as usize,
    s2616 as *const () as usize,
    s2617 as *const () as usize,
    s2618 as *const () as usize,
    s2619 as *const () as usize,
    s2620 as *const () as usize,
    s2621 as *const () as usize,
    s2622 as *const () as usize,
    s2623 as *const () as usize,
    s2624 as *const () as usize,
    s2625 as *const () as usize,
    s2626 as *const () as usize,
    s2627 as *const () as usize,
    s2628 as *const () as usize,
    s2629 as *const () as usize,
    s2630 as *const () as usize,
    s2631 as *const () as usize,
    s2632 as *const () as usize,
    s2633 as *const () as usize,
    s2634 as *const () as usize,
    s2635 as *const () as usize,
    s2636 as *const () as usize,
    s2637 as *const () as usize,
    s2638 as *const () as usize,
    s2639 as *const () as usize,
    s2640 as *const () as usize,
    s2641 as *const () as usize,
    s2642 as *const () as usize,
    s2643 as *const () as usize,
    s2644 as *const () as usize,
    s2645 as *const () as usize,
    s2646 as *const () as usize,
    s2647 as *const () as usize,
    s2648 as *const () as usize,
    s2649 as *const () as usize,
    s2650 as *const () as usize,
    s2651 as *const () as usize,
    s2652 as *const () as usize,
    s2653 as *const () as usize,
    s2654 as *const () as usize,
    s2655 as *const () as usize,
    s2656 as *const () as usize,
    s2657 as *const () as usize,
    s2658 as *const () as usize,
    s2659 as *const () as usize,
    s2660 as *const () as usize,
    s2661 as *const () as usize,
    s2662 as *const () as usize,
    s2663 as *const () as usize,
    s2664 as *const () as usize,
    s2665 as *const () as usize,
    s2666 as *const () as usize,
    s2667 as *const () as usize,
    s2668 as *const () as usize,
    s2669 as *const () as usize,
    s2670 as *const () as usize,
    s2671 as *const () as usize,
    s2672 as *const () as usize,
    s2673 as *const () as usize,
    s2674 as *const () as usize,
    s2675 as *const () as usize,
    s2676 as *const () as usize,
    s2677 as *const () as usize,
    s2678 as *const () as usize,
    s2679 as *const () as usize,
    s2680 as *const () as usize,
    s2681 as *const () as usize,
    s2682 as *const () as usize,
    s2683 as *const () as usize,
    s2684 as *const () as usize,
    s2685 as *const () as usize,
    s2686 as *const () as usize,
    s2687 as *const () as usize,
    s2688 as *const () as usize,
    s2689 as *const () as usize,
    s2690 as *const () as usize,
    s2691 as *const () as usize,
    s2692 as *const () as usize,
    s2693 as *const () as usize,
    s2694 as *const () as usize,
    s2695 as *const () as usize,
    s2696 as *const () as usize,
    s2697 as *const () as usize,
    s2698 as *const () as usize,
    s2699 as *const () as usize,
    s2700 as *const () as usize,
    s2701 as *const () as usize,
    s2702 as *const () as usize,
    s2703 as *const () as usize,
    s2704 as *const () as usize,
    s2705 as *const () as usize,
    s2706 as *const () as usize,
    s2707 as *const () as usize,
    s2708 as *const () as usize,
    s2709 as *const () as usize,
    s2710 as *const () as usize,
    s2711 as *const () as usize,
    s2712 as *const () as usize,
    s2713 as *const () as usize,
    s2714 as *const () as usize,
    s2715 as *const () as usize,
    s2716 as *const () as usize,
    s2717 as *const () as usize,
    s2718 as *const () as usize,
    s2719 as *const () as usize,
    s2720 as *const () as usize,
    s2721 as *const () as usize,
    s2722 as *const () as usize,
    s2723 as *const () as usize,
    s2724 as *const () as usize,
    s2725 as *const () as usize,
    s2726 as *const () as usize,
    s2727 as *const () as usize,
    s2728 as *const () as usize,
    s2729 as *const () as usize,
    s2730 as *const () as usize,
    s2731 as *const () as usize,
    s2732 as *const () as usize,
    s2733 as *const () as usize,
    s2734 as *const () as usize,
    s2735 as *const () as usize,
    s2736 as *const () as usize,
    s2737 as *const () as usize,
    s2738 as *const () as usize,
    s2739 as *const () as usize,
    s2740 as *const () as usize,
    s2741 as *const () as usize,
    s2742 as *const () as usize,
    s2743 as *const () as usize,
    s2744 as *const () as usize,
    s2745 as *const () as usize,
    s2746 as *const () as usize,
    s2747 as *const () as usize,
    s2748 as *const () as usize,
    s2749 as *const () as usize,
    s2750 as *const () as usize,
    s2751 as *const () as usize,
    s2752 as *const () as usize,
    s2753 as *const () as usize,
    s2754 as *const () as usize,
    s2755 as *const () as usize,
    s2756 as *const () as usize,
    s2757 as *const () as usize,
    s2758 as *const () as usize,
    s2759 as *const () as usize,
    s2760 as *const () as usize,
    s2761 as *const () as usize,
    s2762 as *const () as usize,
    s2763 as *const () as usize,
    s2764 as *const () as usize,
    s2765 as *const () as usize,
    s2766 as *const () as usize,
    s2767 as *const () as usize,
    s2768 as *const () as usize,
    s2769 as *const () as usize,
    s2770 as *const () as usize,
    s2771 as *const () as usize,
    s2772 as *const () as usize,
    s2773 as *const () as usize,
    s2774 as *const () as usize,
    s2775 as *const () as usize,
    s2776 as *const () as usize,
    s2777 as *const () as usize,
    s2778 as *const () as usize,
    s2779 as *const () as usize,
    s2780 as *const () as usize,
    s2781 as *const () as usize,
    s2782 as *const () as usize,
    s2783 as *const () as usize,
    s2784 as *const () as usize,
    s2785 as *const () as usize,
    s2786 as *const () as usize,
    s2787 as *const () as usize,
    s2788 as *const () as usize,
    s2789 as *const () as usize,
    s2790 as *const () as usize,
    s2791 as *const () as usize,
    s2792 as *const () as usize,
    s2793 as *const () as usize,
    s2794 as *const () as usize,
    s2795 as *const () as usize,
    s2796 as *const () as usize,
    s2797 as *const () as usize,
    s2798 as *const () as usize,
    s2799 as *const () as usize,
    s2800 as *const () as usize,
    s2801 as *const () as usize,
    s2802 as *const () as usize,
    s2803 as *const () as usize,
    s2804 as *const () as usize,
    s2805 as *const () as usize,
    s2806 as *const () as usize,
    s2807 as *const () as usize,
    s2808 as *const () as usize,
    s2809 as *const () as usize,
    s2810 as *const () as usize,
    s2811 as *const () as usize,
    s2812 as *const () as usize,
    s2813 as *const () as usize,
    s2814 as *const () as usize,
    s2815 as *const () as usize,
    s2816 as *const () as usize,
    s2817 as *const () as usize,
    s2818 as *const () as usize,
    s2819 as *const () as usize,
    s2820 as *const () as usize,
    s2821 as *const () as usize,
    s2822 as *const () as usize,
    s2823 as *const () as usize,
    s2824 as *const () as usize,
    s2825 as *const () as usize,
    s2826 as *const () as usize,
    s2827 as *const () as usize,
    s2828 as *const () as usize,
    s2829 as *const () as usize,
    s2830 as *const () as usize,
    s2831 as *const () as usize,
    s2832 as *const () as usize,
    s2833 as *const () as usize,
    s2834 as *const () as usize,
    s2835 as *const () as usize,
    s2836 as *const () as usize,
    s2837 as *const () as usize,
    s2838 as *const () as usize,
    s2839 as *const () as usize,
    s2840 as *const () as usize,
    s2841 as *const () as usize,
    s2842 as *const () as usize,
    s2843 as *const () as usize,
    s2844 as *const () as usize,
    s2845 as *const () as usize,
    s2846 as *const () as usize,
    s2847 as *const () as usize,
    s2848 as *const () as usize,
    s2849 as *const () as usize,
    s2850 as *const () as usize,
    s2851 as *const () as usize,
    s2852 as *const () as usize,
    s2853 as *const () as usize,
    s2854 as *const () as usize,
    s2855 as *const () as usize,
    s2856 as *const () as usize,
    s2857 as *const () as usize,
    s2858 as *const () as usize,
    s2859 as *const () as usize,
    s2860 as *const () as usize,
    s2861 as *const () as usize,
    s2862 as *const () as usize,
    s2863 as *const () as usize,
    s2864 as *const () as usize,
    s2865 as *const () as usize,
    s2866 as *const () as usize,
    s2867 as *const () as usize,
    s2868 as *const () as usize,
    s2869 as *const () as usize,
    s2870 as *const () as usize,
    s2871 as *const () as usize,
    s2872 as *const () as usize,
    s2873 as *const () as usize,
    s2874 as *const () as usize,
    s2875 as *const () as usize,
    s2876 as *const () as usize,
    s2877 as *const () as usize,
    s2878 as *const () as usize,
    s2879 as *const () as usize,
    s2880 as *const () as usize,
    s2881 as *const () as usize,
    s2882 as *const () as usize,
    s2883 as *const () as usize,
    s2884 as *const () as usize,
    s2885 as *const () as usize,
    s2886 as *const () as usize,
    s2887 as *const () as usize,
    s2888 as *const () as usize,
    s2889 as *const () as usize,
    s2890 as *const () as usize,
    s2891 as *const () as usize,
    s2892 as *const () as usize,
    s2893 as *const () as usize,
    s2894 as *const () as usize,
    s2895 as *const () as usize,
    s2896 as *const () as usize,
    s2897 as *const () as usize,
    s2898 as *const () as usize,
    s2899 as *const () as usize,
    s2900 as *const () as usize,
    s2901 as *const () as usize,
    s2902 as *const () as usize,
    s2903 as *const () as usize,
    s2904 as *const () as usize,
    s2905 as *const () as usize,
    s2906 as *const () as usize,
    s2907 as *const () as usize,
    s2908 as *const () as usize,
    s2909 as *const () as usize,
    s2910 as *const () as usize,
    s2911 as *const () as usize,
    s2912 as *const () as usize,
    s2913 as *const () as usize,
    s2914 as *const () as usize,
    s2915 as *const () as usize,
    s2916 as *const () as usize,
    s2917 as *const () as usize,
    s2918 as *const () as usize,
    s2919 as *const () as usize,
    s2920 as *const () as usize,
    s2921 as *const () as usize,
    s2922 as *const () as usize,
    s2923 as *const () as usize,
    s2924 as *const () as usize,
    s2925 as *const () as usize,
    s2926 as *const () as usize,
    s2927 as *const () as usize,
    s2928 as *const () as usize,
    s2929 as *const () as usize,
    s2930 as *const () as usize,
    s2931 as *const () as usize,
    s2932 as *const () as usize,
    s2933 as *const () as usize,
    s2934 as *const () as usize,
    s2935 as *const () as usize,
    s2936 as *const () as usize,
    s2937 as *const () as usize,
    s2938 as *const () as usize,
    s2939 as *const () as usize,
    s2940 as *const () as usize,
    s2941 as *const () as usize,
    s2942 as *const () as usize,
    s2943 as *const () as usize,
    s2944 as *const () as usize,
    s2945 as *const () as usize,
    s2946 as *const () as usize,
    s2947 as *const () as usize,
    s2948 as *const () as usize,
    s2949 as *const () as usize,
    s2950 as *const () as usize,
    s2951 as *const () as usize,
    s2952 as *const () as usize,
    s2953 as *const () as usize,
    s2954 as *const () as usize,
    s2955 as *const () as usize,
    s2956 as *const () as usize,
    s2957 as *const () as usize,
    s2958 as *const () as usize,
    s2959 as *const () as usize,
    s2960 as *const () as usize,
    s2961 as *const () as usize,
    s2962 as *const () as usize,
    s2963 as *const () as usize,
    s2964 as *const () as usize,
    s2965 as *const () as usize,
    s2966 as *const () as usize,
    s2967 as *const () as usize,
    s2968 as *const () as usize,
    s2969 as *const () as usize,
    s2970 as *const () as usize,
    s2971 as *const () as usize,
    s2972 as *const () as usize,
    s2973 as *const () as usize,
    s2974 as *const () as usize,
    s2975 as *const () as usize,
    s2976 as *const () as usize,
    s2977 as *const () as usize,
    s2978 as *const () as usize,
    s2979 as *const () as usize,
    s2980 as *const () as usize,
    s2981 as *const () as usize,
    s2982 as *const () as usize,
    s2983 as *const () as usize,
    s2984 as *const () as usize,
    s2985 as *const () as usize,
    s2986 as *const () as usize,
    s2987 as *const () as usize,
    s2988 as *const () as usize,
    s2989 as *const () as usize,
    s2990 as *const () as usize,
    s2991 as *const () as usize,
    s2992 as *const () as usize,
    s2993 as *const () as usize,
    s2994 as *const () as usize,
    s2995 as *const () as usize,
    s2996 as *const () as usize,
    s2997 as *const () as usize,
    s2998 as *const () as usize,
    s2999 as *const () as usize,
    s3000 as *const () as usize,
    s3001 as *const () as usize,
    s3002 as *const () as usize,
    s3003 as *const () as usize,
    s3004 as *const () as usize,
    s3005 as *const () as usize,
    s3006 as *const () as usize,
    s3007 as *const () as usize,
    s3008 as *const () as usize,
    s3009 as *const () as usize,
    s3010 as *const () as usize,
    s3011 as *const () as usize,
    s3012 as *const () as usize,
    s3013 as *const () as usize,
    s3014 as *const () as usize,
    s3015 as *const () as usize,
    s3016 as *const () as usize,
    s3017 as *const () as usize,
    s3018 as *const () as usize,
    s3019 as *const () as usize,
    s3020 as *const () as usize,
    s3021 as *const () as usize,
    s3022 as *const () as usize,
    s3023 as *const () as usize,
    s3024 as *const () as usize,
    s3025 as *const () as usize,
    s3026 as *const () as usize,
    s3027 as *const () as usize,
    s3028 as *const () as usize,
    s3029 as *const () as usize,
    s3030 as *const () as usize,
    s3031 as *const () as usize,
    s3032 as *const () as usize,
    s3033 as *const () as usize,
    s3034 as *const () as usize,
    s3035 as *const () as usize,
    s3036 as *const () as usize,
    s3037 as *const () as usize,
    s3038 as *const () as usize,
    s3039 as *const () as usize,
    s3040 as *const () as usize,
    s3041 as *const () as usize,
    s3042 as *const () as usize,
    s3043 as *const () as usize,
    s3044 as *const () as usize,
    s3045 as *const () as usize,
    s3046 as *const () as usize,
    s3047 as *const () as usize,
    s3048 as *const () as usize,
    s3049 as *const () as usize,
    s3050 as *const () as usize,
    s3051 as *const () as usize,
    s3052 as *const () as usize,
    s3053 as *const () as usize,
    s3054 as *const () as usize,
    s3055 as *const () as usize,
    s3056 as *const () as usize,
    s3057 as *const () as usize,
    s3058 as *const () as usize,
    s3059 as *const () as usize,
    s3060 as *const () as usize,
    s3061 as *const () as usize,
    s3062 as *const () as usize,
    s3063 as *const () as usize,
    s3064 as *const () as usize,
    s3065 as *const () as usize,
    s3066 as *const () as usize,
    s3067 as *const () as usize,
    s3068 as *const () as usize,
    s3069 as *const () as usize,
    s3070 as *const () as usize,
    s3071 as *const () as usize,
    s3072 as *const () as usize,
    s3073 as *const () as usize,
    s3074 as *const () as usize,
    s3075 as *const () as usize,
    s3076 as *const () as usize,
    s3077 as *const () as usize,
    s3078 as *const () as usize,
    s3079 as *const () as usize,
    s3080 as *const () as usize,
    s3081 as *const () as usize,
    s3082 as *const () as usize,
    s3083 as *const () as usize,
    s3084 as *const () as usize,
    s3085 as *const () as usize,
    s3086 as *const () as usize,
    s3087 as *const () as usize,
    s3088 as *const () as usize,
    s3089 as *const () as usize,
    s3090 as *const () as usize,
    s3091 as *const () as usize,
    s3092 as *const () as usize,
    s3093 as *const () as usize,
    s3094 as *const () as usize,
    s3095 as *const () as usize,
    s3096 as *const () as usize,
    s3097 as *const () as usize,
    s3098 as *const () as usize,
    s3099 as *const () as usize,
    s3100 as *const () as usize,
    s3101 as *const () as usize,
    s3102 as *const () as usize,
    s3103 as *const () as usize,
    s3104 as *const () as usize,
    s3105 as *const () as usize,
    s3106 as *const () as usize,
    s3107 as *const () as usize,
    s3108 as *const () as usize,
    s3109 as *const () as usize,
    s3110 as *const () as usize,
    s3111 as *const () as usize,
    s3112 as *const () as usize,
    s3113 as *const () as usize,
    s3114 as *const () as usize,
    s3115 as *const () as usize,
    s3116 as *const () as usize,
    s3117 as *const () as usize,
    s3118 as *const () as usize,
    s3119 as *const () as usize,
    s3120 as *const () as usize,
    s3121 as *const () as usize,
    s3122 as *const () as usize,
    s3123 as *const () as usize,
    s3124 as *const () as usize,
    s3125 as *const () as usize,
    s3126 as *const () as usize,
    s3127 as *const () as usize,
    s3128 as *const () as usize,
    s3129 as *const () as usize,
    s3130 as *const () as usize,
    s3131 as *const () as usize,
    s3132 as *const () as usize,
    s3133 as *const () as usize,
    s3134 as *const () as usize,
    s3135 as *const () as usize,
    s3136 as *const () as usize,
    s3137 as *const () as usize,
    s3138 as *const () as usize,
    s3139 as *const () as usize,
    s3140 as *const () as usize,
    s3141 as *const () as usize,
    s3142 as *const () as usize,
    s3143 as *const () as usize,
    s3144 as *const () as usize,
    s3145 as *const () as usize,
    s3146 as *const () as usize,
    s3147 as *const () as usize,
    s3148 as *const () as usize,
    s3149 as *const () as usize,
    s3150 as *const () as usize,
    s3151 as *const () as usize,
    s3152 as *const () as usize,
    s3153 as *const () as usize,
    s3154 as *const () as usize,
    s3155 as *const () as usize,
    s3156 as *const () as usize,
    s3157 as *const () as usize,
    s3158 as *const () as usize,
    s3159 as *const () as usize,
    s3160 as *const () as usize,
    s3161 as *const () as usize,
    s3162 as *const () as usize,
    s3163 as *const () as usize,
    s3164 as *const () as usize,
    s3165 as *const () as usize,
    s3166 as *const () as usize,
    s3167 as *const () as usize,
    s3168 as *const () as usize,
    s3169 as *const () as usize,
    s3170 as *const () as usize,
    s3171 as *const () as usize,
    s3172 as *const () as usize,
    s3173 as *const () as usize,
    s3174 as *const () as usize,
    s3175 as *const () as usize,
    s3176 as *const () as usize,
    s3177 as *const () as usize,
    s3178 as *const () as usize,
    s3179 as *const () as usize,
    s3180 as *const () as usize,
    s3181 as *const () as usize,
    s3182 as *const () as usize,
    s3183 as *const () as usize,
    s3184 as *const () as usize,
    s3185 as *const () as usize,
    s3186 as *const () as usize,
    s3187 as *const () as usize,
    s3188 as *const () as usize,
    s3189 as *const () as usize,
    s3190 as *const () as usize,
    s3191 as *const () as usize,
    s3192 as *const () as usize,
    s3193 as *const () as usize,
    s3194 as *const () as usize,
    s3195 as *const () as usize,
    s3196 as *const () as usize,
    s3197 as *const () as usize,
    s3198 as *const () as usize,
    s3199 as *const () as usize,
    s3200 as *const () as usize,
    s3201 as *const () as usize,
    s3202 as *const () as usize,
    s3203 as *const () as usize,
    s3204 as *const () as usize,
    s3205 as *const () as usize,
    s3206 as *const () as usize,
    s3207 as *const () as usize,
    s3208 as *const () as usize,
    s3209 as *const () as usize,
    s3210 as *const () as usize,
    s3211 as *const () as usize,
    s3212 as *const () as usize,
    s3213 as *const () as usize,
    s3214 as *const () as usize,
    s3215 as *const () as usize,
    s3216 as *const () as usize,
    s3217 as *const () as usize,
    s3218 as *const () as usize,
    s3219 as *const () as usize,
    s3220 as *const () as usize,
    s3221 as *const () as usize,
    s3222 as *const () as usize,
    s3223 as *const () as usize,
    s3224 as *const () as usize,
    s3225 as *const () as usize,
    s3226 as *const () as usize,
    s3227 as *const () as usize,
    s3228 as *const () as usize,
    s3229 as *const () as usize,
    s3230 as *const () as usize,
    s3231 as *const () as usize,
    s3232 as *const () as usize,
    s3233 as *const () as usize,
    s3234 as *const () as usize,
    s3235 as *const () as usize,
    s3236 as *const () as usize,
    s3237 as *const () as usize,
    s3238 as *const () as usize,
    s3239 as *const () as usize,
    s3240 as *const () as usize,
    s3241 as *const () as usize,
    s3242 as *const () as usize,
    s3243 as *const () as usize,
    s3244 as *const () as usize,
    s3245 as *const () as usize,
    s3246 as *const () as usize,
    s3247 as *const () as usize,
    s3248 as *const () as usize,
    s3249 as *const () as usize,
    s3250 as *const () as usize,
    s3251 as *const () as usize,
    s3252 as *const () as usize,
    s3253 as *const () as usize,
    s3254 as *const () as usize,
    s3255 as *const () as usize,
    s3256 as *const () as usize,
    s3257 as *const () as usize,
    s3258 as *const () as usize,
    s3259 as *const () as usize,
    s3260 as *const () as usize,
    s3261 as *const () as usize,
    s3262 as *const () as usize,
    s3263 as *const () as usize,
    s3264 as *const () as usize,
    s3265 as *const () as usize,
    s3266 as *const () as usize,
    s3267 as *const () as usize,
    s3268 as *const () as usize,
    s3269 as *const () as usize,
    s3270 as *const () as usize,
    s3271 as *const () as usize,
    s3272 as *const () as usize,
    s3273 as *const () as usize,
    s3274 as *const () as usize,
    s3275 as *const () as usize,
    s3276 as *const () as usize,
    s3277 as *const () as usize,
    s3278 as *const () as usize,
    s3279 as *const () as usize,
    s3280 as *const () as usize,
    s3281 as *const () as usize,
    s3282 as *const () as usize,
    s3283 as *const () as usize,
    s3284 as *const () as usize,
    s3285 as *const () as usize,
    s3286 as *const () as usize,
    s3287 as *const () as usize,
    s3288 as *const () as usize,
    s3289 as *const () as usize,
    s3290 as *const () as usize,
    s3291 as *const () as usize,
    s3292 as *const () as usize,
    s3293 as *const () as usize,
    s3294 as *const () as usize,
    s3295 as *const () as usize,
    s3296 as *const () as usize,
    s3297 as *const () as usize,
    s3298 as *const () as usize,
    s3299 as *const () as usize,
    s3300 as *const () as usize,
    s3301 as *const () as usize,
    s3302 as *const () as usize,
    s3303 as *const () as usize,
    s3304 as *const () as usize,
    s3305 as *const () as usize,
    s3306 as *const () as usize,
    s3307 as *const () as usize,
    s3308 as *const () as usize,
    s3309 as *const () as usize,
    s3310 as *const () as usize,
    s3311 as *const () as usize,
    s3312 as *const () as usize,
    s3313 as *const () as usize,
    s3314 as *const () as usize,
    s3315 as *const () as usize,
    s3316 as *const () as usize,
    s3317 as *const () as usize,
    s3318 as *const () as usize,
    s3319 as *const () as usize,
    s3320 as *const () as usize,
    s3321 as *const () as usize,
    s3322 as *const () as usize,
    s3323 as *const () as usize,
    s3324 as *const () as usize,
    s3325 as *const () as usize,
    s3326 as *const () as usize,
    s3327 as *const () as usize,
    s3328 as *const () as usize,
    s3329 as *const () as usize,
    s3330 as *const () as usize,
    s3331 as *const () as usize,
    s3332 as *const () as usize,
    s3333 as *const () as usize,
    s3334 as *const () as usize,
    s3335 as *const () as usize,
    s3336 as *const () as usize,
    s3337 as *const () as usize,
    s3338 as *const () as usize,
    s3339 as *const () as usize,
    s3340 as *const () as usize,
    s3341 as *const () as usize,
    s3342 as *const () as usize,
    s3343 as *const () as usize,
    s3344 as *const () as usize,
    s3345 as *const () as usize,
    s3346 as *const () as usize,
    s3347 as *const () as usize,
    s3348 as *const () as usize,
    s3349 as *const () as usize,
    s3350 as *const () as usize,
    s3351 as *const () as usize,
    s3352 as *const () as usize,
    s3353 as *const () as usize,
    s3354 as *const () as usize,
    s3355 as *const () as usize,
    s3356 as *const () as usize,
    s3357 as *const () as usize,
    s3358 as *const () as usize,
    s3359 as *const () as usize,
    s3360 as *const () as usize,
    s3361 as *const () as usize,
    s3362 as *const () as usize,
    s3363 as *const () as usize,
    s3364 as *const () as usize,
    s3365 as *const () as usize,
    s3366 as *const () as usize,
    s3367 as *const () as usize,
    s3368 as *const () as usize,
    s3369 as *const () as usize,
    s3370 as *const () as usize,
    s3371 as *const () as usize,
    s3372 as *const () as usize,
    s3373 as *const () as usize,
    s3374 as *const () as usize,
    s3375 as *const () as usize,
    s3376 as *const () as usize,
    s3377 as *const () as usize,
    s3378 as *const () as usize,
    s3379 as *const () as usize,
    s3380 as *const () as usize,
    s3381 as *const () as usize,
    s3382 as *const () as usize,
    s3383 as *const () as usize,
    s3384 as *const () as usize,
    s3385 as *const () as usize,
    s3386 as *const () as usize,
    s3387 as *const () as usize,
    s3388 as *const () as usize,
    s3389 as *const () as usize,
    s3390 as *const () as usize,
    s3391 as *const () as usize,
    s3392 as *const () as usize,
    s3393 as *const () as usize,
    s3394 as *const () as usize,
    s3395 as *const () as usize,
    s3396 as *const () as usize,
    s3397 as *const () as usize,
    s3398 as *const () as usize,
    s3399 as *const () as usize,
    s3400 as *const () as usize,
    s3401 as *const () as usize,
    s3402 as *const () as usize,
    s3403 as *const () as usize,
    s3404 as *const () as usize,
    s3405 as *const () as usize,
    s3406 as *const () as usize,
    s3407 as *const () as usize,
    s3408 as *const () as usize,
    s3409 as *const () as usize,
    s3410 as *const () as usize,
    s3411 as *const () as usize,
    s3412 as *const () as usize,
    s3413 as *const () as usize,
    s3414 as *const () as usize,
    s3415 as *const () as usize,
    s3416 as *const () as usize,
    s3417 as *const () as usize,
    s3418 as *const () as usize,
    s3419 as *const () as usize,
    s3420 as *const () as usize,
    s3421 as *const () as usize,
    s3422 as *const () as usize,
    s3423 as *const () as usize,
    s3424 as *const () as usize,
    s3425 as *const () as usize,
    s3426 as *const () as usize,
    s3427 as *const () as usize,
    s3428 as *const () as usize,
    s3429 as *const () as usize,
    s3430 as *const () as usize,
    s3431 as *const () as usize,
    s3432 as *const () as usize,
    s3433 as *const () as usize,
    s3434 as *const () as usize,
    s3435 as *const () as usize,
    s3436 as *const () as usize,
    s3437 as *const () as usize,
    s3438 as *const () as usize,
    s3439 as *const () as usize,
    s3440 as *const () as usize,
    s3441 as *const () as usize,
    s3442 as *const () as usize,
    s3443 as *const () as usize,
    s3444 as *const () as usize,
    s3445 as *const () as usize,
    s3446 as *const () as usize,
    s3447 as *const () as usize,
    s3448 as *const () as usize,
    s3449 as *const () as usize,
    s3450 as *const () as usize,
    s3451 as *const () as usize,
    s3452 as *const () as usize,
    s3453 as *const () as usize,
    s3454 as *const () as usize,
    s3455 as *const () as usize,
    s3456 as *const () as usize,
    s3457 as *const () as usize,
    s3458 as *const () as usize,
    s3459 as *const () as usize,
    s3460 as *const () as usize,
    s3461 as *const () as usize,
    s3462 as *const () as usize,
    s3463 as *const () as usize,
    s3464 as *const () as usize,
    s3465 as *const () as usize,
    s3466 as *const () as usize,
    s3467 as *const () as usize,
    s3468 as *const () as usize,
    s3469 as *const () as usize,
    s3470 as *const () as usize,
    s3471 as *const () as usize,
    s3472 as *const () as usize,
    s3473 as *const () as usize,
    s3474 as *const () as usize,
    s3475 as *const () as usize,
    s3476 as *const () as usize,
    s3477 as *const () as usize,
    s3478 as *const () as usize,
    s3479 as *const () as usize,
    s3480 as *const () as usize,
    s3481 as *const () as usize,
    s3482 as *const () as usize,
    s3483 as *const () as usize,
    s3484 as *const () as usize,
    s3485 as *const () as usize,
    s3486 as *const () as usize,
    s3487 as *const () as usize,
    s3488 as *const () as usize,
    s3489 as *const () as usize,
    s3490 as *const () as usize,
    s3491 as *const () as usize,
    s3492 as *const () as usize,
    s3493 as *const () as usize,
    s3494 as *const () as usize,
    s3495 as *const () as usize,
    s3496 as *const () as usize,
    s3497 as *const () as usize,
    s3498 as *const () as usize,
    s3499 as *const () as usize,
    s3500 as *const () as usize,
    s3501 as *const () as usize,
    s3502 as *const () as usize,
    s3503 as *const () as usize,
    s3504 as *const () as usize,
    s3505 as *const () as usize,
    s3506 as *const () as usize,
    s3507 as *const () as usize,
    s3508 as *const () as usize,
    s3509 as *const () as usize,
    s3510 as *const () as usize,
    s3511 as *const () as usize,
    s3512 as *const () as usize,
    s3513 as *const () as usize,
    s3514 as *const () as usize,
    s3515 as *const () as usize,
    s3516 as *const () as usize,
    s3517 as *const () as usize,
    s3518 as *const () as usize,
    s3519 as *const () as usize,
    s3520 as *const () as usize,
    s3521 as *const () as usize,
    s3522 as *const () as usize,
    s3523 as *const () as usize,
    s3524 as *const () as usize,
    s3525 as *const () as usize,
    s3526 as *const () as usize,
    s3527 as *const () as usize,
    s3528 as *const () as usize,
    s3529 as *const () as usize,
    s3530 as *const () as usize,
    s3531 as *const () as usize,
    s3532 as *const () as usize,
    s3533 as *const () as usize,
    s3534 as *const () as usize,
    s3535 as *const () as usize,
    s3536 as *const () as usize,
    s3537 as *const () as usize,
    s3538 as *const () as usize,
    s3539 as *const () as usize,
    s3540 as *const () as usize,
    s3541 as *const () as usize,
    s3542 as *const () as usize,
    s3543 as *const () as usize,
    s3544 as *const () as usize,
    s3545 as *const () as usize,
    s3546 as *const () as usize,
    s3547 as *const () as usize,
    s3548 as *const () as usize,
    s3549 as *const () as usize,
    s3550 as *const () as usize,
    s3551 as *const () as usize,
    s3552 as *const () as usize,
    s3553 as *const () as usize,
    s3554 as *const () as usize,
    s3555 as *const () as usize,
    s3556 as *const () as usize,
    s3557 as *const () as usize,
    s3558 as *const () as usize,
    s3559 as *const () as usize,
    s3560 as *const () as usize,
    s3561 as *const () as usize,
    s3562 as *const () as usize,
    s3563 as *const () as usize,
    s3564 as *const () as usize,
    s3565 as *const () as usize,
    s3566 as *const () as usize,
    s3567 as *const () as usize,
    s3568 as *const () as usize,
    s3569 as *const () as usize,
    s3570 as *const () as usize,
    s3571 as *const () as usize,
    s3572 as *const () as usize,
    s3573 as *const () as usize,
    s3574 as *const () as usize,
    s3575 as *const () as usize,
    s3576 as *const () as usize,
    s3577 as *const () as usize,
] }
