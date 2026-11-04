using Grubs;
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

		var grubs = players.SelectMany( p => p.Grubs.Where( g => g.IsValid() ).Select( g =>
			$"{g.GameObject.Name} of {p.GameObject.Name} at {g.WorldPosition.x:0},{g.WorldPosition.y:0},{g.WorldPosition.z:0} health {g.Health?.CurrentHealth:0}" ) );
		Log.Info( $"shimwatch: {ffa.State}, turn of {ffa.ActivePlayer?.GameObject.Name ?? "no one"} ({ffa.ActivePlayer?.ActiveGrub?.GameObject.Name ?? "no grub"}), {(float)ffa.TimeUntilTurnOver:0}s left{(ffa.TurnIsChanging ? ", changing" : "")}; grubs: {string.Join( "; ", grubs )}" );
	}
}
