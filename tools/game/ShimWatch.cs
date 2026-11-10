/// <summary>
/// For any game hosted here with no one at its screen: says every two
/// seconds what its scene holds (objects, models, players connected), on
/// the server and on each player, and on a player saves what the scene's
/// camera sees every ten seconds (CameraComponent.RenderToBitmap), so a run
/// can be looked at afterwards. tools/game.sh puts it in a game's startup
/// scene.
/// </summary>
public sealed class ShimWatch : Component
{
	TimeSince _last;
	int _ticks;

	protected override void OnUpdate()
	{
		if ( _last < 2 ) return;
		_last = 0;
		_ticks++;
		var who = Networking.IsHost ? "host" : "player";
		Log.Info( $"shimwatch {_ticks}: {who}, scene {Scene.Source?.ResourcePath ?? "(none)"}, {Scene.GetAllObjects( true ).Count()} objects, {Scene.GetAllComponents<ModelRenderer>().Count()} models drawn, {Connection.All.Count} connected" );
		if ( Networking.IsHost || _ticks % 5 != 0 ) return;
		var camera = Scene.Camera ?? Scene.GetAllComponents<CameraComponent>().FirstOrDefault();
		if ( !camera.IsValid() ) { Log.Info( "shimwatch view: no camera" ); return; }
		var bitmap = new Bitmap( 960, 540 );
		var exposure = camera.AutoExposure.Enabled;
		camera.AutoExposure.Enabled = false;
		camera.RenderToBitmap( bitmap, false );
		camera.AutoExposure.Enabled = exposure;
		var file = $"shimview_{_ticks * 2}.png";
		FileSystem.Data.WriteAllBytes( file, bitmap.ToPng() );
		Log.Info( $"shimwatch view: {camera.GameObject.Name} at {camera.WorldPosition}, to {FileSystem.Data.GetFullPath( file )}" );
	}
}
