using Grubs;
using Grubs.Helpers;
using Grubs.Systems.GameMode;
using Grubs.Systems.Pawn;
using Sandbox.Sdf;

/// <summary>
/// For a Grubs server with no one to press its buttons: once a player has
/// sat in the lobby for a few seconds, starts the match as the lobby's
/// SANDBOX button does for a host alone (the game kept alive with one
/// player, ammo without end); a dedicated server is the host, so no player
/// is shown START or SANDBOX. Then says every two seconds what the match is
/// doing: its state, whose turn and how long is left, and each grub (where,
/// its health, whether it's on the ground). And on the server and on each
/// player, how many times the terrain has been carved (Sdf2DWorld's
/// modifications: an explosion's crater, made by the host, sent to players).
/// </summary>
public sealed class ShimWatch : Component
{
	TimeSince _last;
	TimeSince _seen;
	bool _waiting;

	int _carved = -1;
	int _views;
	int _playing;
	bool _blown;

	// what a player sees, every ten seconds: the scene's camera drawn into a
	// bitmap and saved (the terrain's and the effects' materials are the
	// game's own shader graphs, translated: engine2/src/assets/shadergraph.rs)
	void View()
	{
		if ( Networking.IsHost || ++_views % 5 != 0 ) return;
		var camera = Scene.Camera ?? Scene.GetAllComponents<CameraComponent>().FirstOrDefault();
		if ( !camera.IsValid() ) { Log.Info( "shimwatch view: no camera" ); return; }
		var bitmap = new Bitmap( 960, 540 );
		var exposure = camera.AutoExposure.Enabled;
		camera.AutoExposure.Enabled = false;
		camera.RenderToBitmap( bitmap, false );
		camera.AutoExposure.Enabled = exposure;
		var file = $"shimview_{_views * 2}.png";
		FileSystem.Data.WriteAllBytes( file, bitmap.ToPng() );
		Log.Info( $"shimwatch view: {camera.GameObject.Name} at {camera.WorldPosition}, to {FileSystem.Data.GetFullPath( file )}" );
	}

	protected override void OnUpdate()
	{
		if ( _last < 2 ) return;
		_last = 0;
		var carved = Scene.GetAllComponents<Sdf2DWorld>().Sum( w => w.ModificationCount );
		if ( carved != _carved )
		{
			Log.Info( $"shimwatch: terrain carved {carved} times ({(Networking.IsHost ? "host" : "player")})" );
			_carved = carved;
		}
		View();
		if ( !Networking.IsHost ) return;
		if ( BaseGameMode.Current is not FreeForAll ffa ) { Log.Info( "shimwatch: no Free For All mode yet" ); return; }

		var players = Player.All.ToList();
		if ( ffa.State == FreeForAllState.Lobby )
		{
			if ( players.Count == 0 ) { _waiting = false; return; }
			if ( !_waiting ) { _waiting = true; _seen = 0; }
			if ( _seen < 6 ) { Log.Info( $"shimwatch: lobby, {players.Count} players" ); return; }
			GrubsConfig.KeepGameAlive = true;
			GrubsConfig.InfiniteAmmo = true;
			ffa.IsSandboxMode = true;
			ffa.Start();
			Log.Info( $"shimwatch: started, as SANDBOX does, with {players.Count} players: state {ffa.State}" );
			return;
		}

		// once, ten seconds into the match: an explosion under the first grub,
		// as a weapon's would be: a crater, and the scorch around it (the
		// terrain's chunks are told where through their renderers' attributes)
		if ( !_blown && ffa.State == FreeForAllState.Playing && ++_playing == 5 )
		{
			var grub = players.SelectMany( p => p.Grubs ).FirstOrDefault( g => g.IsValid() );
			if ( grub.IsValid() )
			{
				_blown = true;
				var at = grub.WorldPosition + Vector3.Right * 40f - Vector3.Up * 24f;
				ExplosionHelper.Instance.Explode( this, at, 96f, 0f, Guid.Empty, "shimwatch" );
				Log.Info( $"shimwatch: an explosion at {at.x:0},{at.y:0},{at.z:0}" );
			}
		}

		var grubs = players.SelectMany( p => p.Grubs.Where( g => g.IsValid() ).Select( g =>
			$"{g.GameObject.Name} of {p.GameObject.Name} at {g.WorldPosition.x:0},{g.WorldPosition.y:0},{g.WorldPosition.z:0} health {g.Health?.CurrentHealth:0}" ) );
		Log.Info( $"shimwatch: {ffa.State}, turn of {ffa.ActivePlayer?.GameObject.Name ?? "no one"} ({ffa.ActivePlayer?.ActiveGrub?.GameObject.Name ?? "no grub"}), {(float)ffa.TimeUntilTurnOver:0}s left{(ffa.TurnIsChanging ? ", changing" : "")}; grubs: {string.Join( "; ", grubs )}" );
	}
}
