/// <summary>
/// Says, once a second, what physics is doing: how many bodies are awake,
/// where they are, what a trace down from above hits, and how many
/// collisions the scene has heard.
/// </summary>
public sealed class Watch : Component
{
	public static int Collisions;
	TimeSince _last;
	int _ticks;

	protected override void OnStart()
	{
		foreach ( var rb in Scene.GetAllComponents<Rigidbody>() )
			rb.GameObject.Components.Create<Bump>();
		Log.Info( $"watch: {Scene.GetAllComponents<Rigidbody>().Count()} rigidbodies" );
	}

	protected override void OnFixedUpdate()
	{
		if ( _last < 1 ) return;
		_last = 0;
		_ticks++;
		var bodies = Scene.GetAllComponents<Rigidbody>().ToList();
		var zs = bodies.Select( b => b.WorldPosition.z ).ToList();
		var awake = bodies.Count( b => !b.PhysicsBody.Sleeping );
		var fastest = bodies.Max( b => b.Velocity.Length );
		var tr = Scene.Trace.Ray( new Vector3( 386, -117, 2000 ), new Vector3( 386, -117, -2000 ) ).Run();
		Log.Info( $"watch {_ticks}: awake {awake}/{bodies.Count} z {zs.Min():0.0}..{zs.Max():0.0} fastest {fastest:0.0} collisions {Collisions} trace {(tr.Hit ? $"hit {tr.GameObject?.Name} at z {tr.HitPosition.z:0.0} normal {tr.Normal}" : "missed")}" );
		if ( _ticks == 6 )
		{
			var first = bodies.First();
			first.ApplyImpulse( Vector3.Up * first.Mass * 600 );
			Log.Info( $"watch: kicked {first.GameObject.Name} (mass {first.Mass:0.0}) up" );
		}
	}
}

public sealed class Bump : Component, Component.ICollisionListener
{
	public void OnCollisionStart( Collision c ) => Watch.Collisions++;
}
