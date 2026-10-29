/// <summary>
/// Says once a second what the game's world is doing, for a server with no
/// screen: each player (where, standing on what), the bodies (how many,
/// how many awake, how high), the joints.
/// </summary>
public sealed class ShimWatch : Component
{
	TimeSince _last;
	int _ticks;

	protected override void OnFixedUpdate()
	{
		if ( _last < 1 ) return;
		_last = 0;
		_ticks++;

		var players = Scene.GetAllComponents<PlayerController>()
			.Select( p => $"{p.GameObject.Name} at {p.WorldPosition.x:0},{p.WorldPosition.y:0},{p.WorldPosition.z:0} speed {p.Velocity.Length:0} {(p.IsOnGround ? $"on {p.GroundObject?.Name}" : "in the air")}" );
		var bodies = Scene.GetAllComponents<Rigidbody>().Where( b => b.PhysicsBody.IsValid() ).ToList();
		var awake = bodies.Count( b => !b.PhysicsBody.Sleeping );
		var zs = bodies.Select( b => b.WorldPosition.z ).DefaultIfEmpty( 0 ).ToList();
		var joints = Scene.GetAllComponents<Joint>().Count();
		var colliders = Scene.GetAllComponents<Collider>().Count();

		if ( _ticks == 3 )
		{
			// once: what fell, and what every collider is made of
			foreach ( var b in bodies.Where( b => b.WorldPosition.z < -100 ) )
				Log.Info( $"shimwatch: {b.GameObject.Name} fell, its colliders {string.Join( ", ", b.GameObject.Components.GetAll<Collider>( FindMode.EverythingInSelfAndDescendants ).Select( c => c.GetType().Name ) )}" );
			foreach ( var g in Scene.GetAllComponents<Collider>().GroupBy( c => c.GetType().Name ) )
				Log.Info( $"shimwatch: {g.Count()} {g.Key}, sizes {string.Join( " ", g.Select( c => c.LocalBounds.Size.Length.ToString( "0" ) ) )}" );
		}

		Log.Info( $"shimwatch {_ticks}: players [{string.Join( "; ", players )}] | bodies {bodies.Count} awake {awake} z {zs.Min():0}..{zs.Max():0} | joints {joints} | colliders {colliders}" );
	}
}
