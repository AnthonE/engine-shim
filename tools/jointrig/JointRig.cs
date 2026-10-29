/// <summary>
/// Builds five jointed rigs out of s&box's own joint components, away from
/// the cubes, and says once a second what each is doing: a hinged pendulum,
/// a chain of three ball joints, a box on a spring, a box on a vertical
/// slider, and a weld too weak to hold.
/// </summary>
public sealed class JointRig : Component
{
	GameObject _pendulum, _springBox, _sliderBox, _weldBox;
	GameObject[] _links;
	Vector3 _pivot, _springTop, _sliderStart, _chainTop;
	bool _broke;
	TimeSince _last;
	int _ticks;

	GameObject Box( string name, Vector3 at, bool dynamic = true )
	{
		var go = new GameObject( true, name );
		go.WorldPosition = at;
		var c = go.Components.Create<BoxCollider>();
		c.Scale = new Vector3( 20 );
		c.Static = !dynamic;
		if ( dynamic ) go.Components.Create<Rigidbody>();
		return go;
	}

	/// A joint on `on`, at its local `at` and turned `rot`, to `body` (null: the world).
	T Join<T>( GameObject on, Vector3 at, Rotation rot, GameObject body, System.Action<T> setup = null ) where T : Joint, new()
	{
		var a = new GameObject( false, typeof( T ).Name );
		a.SetParent( on, false );
		a.LocalPosition = at;
		a.LocalRotation = rot;
		var j = a.Components.Create<T>();
		j.Body = body;
		setup?.Invoke( j );
		a.Enabled = true;
		return j;
	}

	protected override void OnStart()
	{
		var x = 1000f;

		// a pendulum 100 from its hinge, the hinge's axis along y
		_pivot = new Vector3( x, 0, 400 );
		_pendulum = Box( "Pendulum", _pivot + new Vector3( 100, 0, 0 ) );
		Join<HingeJoint>( _pendulum, new Vector3( -100, 0, 0 ), Rotation.FromRoll( 90 ), null );

		// three links, 50 apart on a diagonal, ball-jointed to the world and each other
		_chainTop = new Vector3( x + 300, 0, 400 );
		_links = new GameObject[3];
		for ( int i = 0; i < 3; i++ )
			_links[i] = Box( $"Link{i}", _chainTop + new Vector3( 35 * (i + 1), 0, -35 * (i + 1) ) );
		Join<BallJoint>( _links[0], new Vector3( -35, 0, 35 ), Rotation.Identity, null );
		for ( int i = 1; i < 3; i++ )
			Join<BallJoint>( _links[i], new Vector3( -35, 0, 35 ), Rotation.Identity, _links[i - 1] );

		// a box on a spring from a fixed hook 150 above it, resting at 100
		_springTop = new Vector3( x + 600, 0, 400 );
		var hook = Box( "Hook", _springTop, dynamic: false );
		_springBox = Box( "SpringBox", _springTop + new Vector3( 0, 0, -150 ) );
		Join<SpringJoint>( _springBox, Vector3.Zero, Rotation.Identity, hook, s =>
		{
			s.RestLength = 100;
			s.MinLength = 0;
			s.MaxLength = 1000;
			s.Frequency = 2;
			s.Damping = 0.7f;
		} );

		// a box on a slider whose axis points down, free to travel 100 either way
		_sliderStart = new Vector3( x + 900, 0, 300 );
		_sliderBox = Box( "SliderBox", _sliderStart + new Vector3( 0, 0, 0 ) );
		_sliderBox.WorldRotation = Rotation.FromYaw( 30 );
		Join<SliderJoint>( _sliderBox, Vector3.Zero, Rotation.FromPitch( 90 ), null, s =>
		{
			s.MinLength = -100;
			s.MaxLength = 100;
		} );

		// a weld that can't take the box's weight
		_weldBox = Box( "WeldBox", new Vector3( x + 1200, 0, 300 ) );
		Join<FixedJoint>( _weldBox, Vector3.Zero, Rotation.Identity, null, f =>
		{
			f.BreakForce = 1;
			f.OnBreak = () => { _broke = true; Log.Info( "jointrig: the weld broke" ); };
		} );

		Log.Info( "jointrig: built" );
	}

	protected override void OnFixedUpdate()
	{
		if ( _last < 1 ) return;
		_last = 0;
		_ticks++;

		var p = _pendulum.WorldPosition - _pivot;
		var hinge = _pendulum.Components.GetInDescendants<HingeJoint>();
		var chain = string.Join( " ", _links.Select( ( l, i ) => (l.WorldPosition - (i == 0 ? _chainTop + new Vector3( 0, 0, 0 ) : _links[i - 1].WorldPosition)).Length.ToString( "0.0" ) ) );
		var spring = (_springTop - _springBox.WorldPosition).Length;
		var s = _sliderBox.WorldPosition - _sliderStart;
		Log.Info( $"jointrig {_ticks}: pendulum r {p.Length:0.0} y {p.y:0.00} z {p.z:0.0} angle {hinge?.Angle:0.0}"
			+ $" | chain {chain} bottom z {_links[2].WorldPosition.z:0.0}"
			+ $" | spring {spring:0.0}"
			+ $" | slider dz {s.z:0.0} dxy {s.WithZ( 0 ).Length:0.00}"
			+ $" | weld {(_broke ? "broken" : "holding")} z {_weldBox.WorldPosition.z:0.0}" );
	}
}
