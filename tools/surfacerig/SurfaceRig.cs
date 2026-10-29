/// <summary>
/// Shapes made of core's surfaces, on a floor of the default one, away from
/// the cubes, and once a second what each is doing: balls dropped, which
/// bounce as their surface does; boxes pushed, which slide as far as their
/// friction lets them (one with a friction of its own over its surface's);
/// boxes of one size, which weigh what their surface does; and a trace
/// down onto each, which says what it hit.
/// </summary>
public sealed class SurfaceRig : Component
{
	const float FloorTop = 10;
	static readonly string[] Bouncers = { "default", "rubber", "metal.sheet" };
	static readonly string[] Sliders = { "default", "slippy_wheels", "slippy_wheels+friction" };
	static readonly string[] Weights = { "default", "wood", "metal", "fabric" };

	readonly List<(string name, GameObject go)> _balls = new(), _slides = new(), _weights = new();
	readonly Dictionary<GameObject, float> _peak = new();
	readonly HashSet<GameObject> _bounced = new();
	TimeSince _last;
	int _ticks;

	static Surface Load( string name ) => ResourceLibrary.Get<Surface>( $"surfaces/{name}.surface" );

	GameObject Make( string name, Vector3 at, bool sphere, string surface, bool dynamic = true )
	{
		var go = new GameObject( true, name );
		go.WorldPosition = at;
		Collider c;
		if ( sphere ) { var s = go.Components.Create<SphereCollider>(); s.Radius = 10; c = s; }
		else { var b = go.Components.Create<BoxCollider>(); b.Scale = new Vector3( 20 ); c = b; }
		var friction = surface.EndsWith( "+friction" );
		surface = surface.Replace( "+friction", "" );
		c.Surface = Load( surface );
		if ( c.Surface is null ) Log.Warning( $"surfacerig: no surfaces/{surface}.surface" );
		if ( friction ) c.Friction = 1;
		c.Static = !dynamic;
		if ( dynamic ) go.Components.Create<Rigidbody>();
		return go;
	}

	protected override void OnStart()
	{
		var y = -3000f;
		var floor = new GameObject( true, "SurfaceFloor" );
		floor.WorldPosition = new Vector3( 0, y, 0 );
		var f = floor.Components.Create<BoxCollider>();
		f.Scale = new Vector3( 4000, 1200, 20 );
		f.Static = true;

		for ( int i = 0; i < Bouncers.Length; i++ )
			_balls.Add( (Bouncers[i], Make( $"Ball {Bouncers[i]}", new Vector3( -1500 + 100 * i, y - 400, 200 ), true, Bouncers[i] )) );
		for ( int i = 0; i < Sliders.Length; i++ )
		{
			var go = Make( $"Slide {Sliders[i]}", new Vector3( -1500, y - 100 + 100 * i, FloorTop + 10 ), false, Sliders[i] );
			go.Components.Get<Rigidbody>().Velocity = new Vector3( 300, 0, 0 );
			_slides.Add( (Sliders[i], go) );
		}
		for ( int i = 0; i < Weights.Length; i++ )
			_weights.Add( (Weights[i], Make( $"Weight {Weights[i]}", new Vector3( 1000 + 100 * i, y + 400, FloorTop + 10 ), false, Weights[i] )) );

		Log.Info( $"surfacerig: built; {string.Join( " ", Bouncers.Concat( Weights ).Distinct().Select( n => $"{n}={(Load( n ) is { } s ? $"{s.Friction}/{s.Elasticity}/{s.Density}" : "missing")}" ) )}" );
	}

	protected override void OnFixedUpdate()
	{
		foreach ( var (_, go) in _balls )
		{
			var rb = go.Components.Get<Rigidbody>();
			if ( rb.Velocity.z > 1 ) _bounced.Add( go );
			if ( _bounced.Contains( go ) ) if ( go.WorldPosition.z > _peak.GetValueOrDefault( go ) ) _peak[go] = go.WorldPosition.z;
		}

		if ( _last < 1 ) return;
		_last = 0;
		_ticks++;

		string Hit( GameObject go )
		{
			var tr = Scene.Trace.Ray( go.WorldPosition + Vector3.Up * 100, go.WorldPosition ).Run();
			return tr.Hit ? tr.Surface?.ResourceName ?? "none" : "miss";
		}

		var rest = FloorTop + 10;
		Log.Info( $"surfacerig {_ticks}: bounce " + string.Join( " ", _balls.Select( b => $"{b.name} {(_peak.TryGetValue( b.go, out var p ) ? p - rest : 0):0.0}" ) )
			+ " | slide " + string.Join( " ", _slides.Select( s => $"{s.name} {s.go.WorldPosition.x + 1500:0.0} v {s.go.Components.Get<Rigidbody>().Velocity.Length:0.0}" ) )
			+ " | mass " + string.Join( " ", _weights.Select( w => $"{w.name} {w.go.Components.Get<Rigidbody>().Mass:0.0}" ) )
			+ " | trace " + string.Join( " ", _weights.Select( w => $"{w.name}:{Hit( w.go )}" ) ) );
	}
}
