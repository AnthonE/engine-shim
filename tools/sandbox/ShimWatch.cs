/// <summary>
/// Says once a second what the game's world is doing, for a server with no
/// screen: each player (where, standing on what), the bodies (how many,
/// how many awake, how high), the joints. And shoots at a model's
/// hitboxes: a skinned model from s&box's download cache, with
/// ModelHitboxes, hit by traces aimed at its bones, then turned and moved,
/// then playing a sequence. And breaks a prop, and follows its gibs; and
/// sets off an explosive one, and sees what it does to its neighbours.
/// </summary>
public sealed class ShimWatch : Component
{
	TimeSince _last;
	int _ticks;
	Prop _broken, _bomb;
	Vector3 _blast;
	Dictionary<Rigidbody, Vector3> _near;
	bool _blew;

	protected override void OnFixedUpdate()
	{
		// the step after the blast: how fast each neighbour goes, against what
		// RadiusDamage's force (1e10 / (d² + 1), for one step) predicts
		if ( _blew )
		{
			_blew = false;
			Log.Info( "shimwatch blast 0: " + string.Join( ", ", _near.Keys.Where( b => b.IsValid() ).Select( b =>
			{
				var d = _near[b].Distance( _blast );
				var predicted = 10000000000f / (d * d + 1) / b.Mass * Time.Delta;
				return $"{b.GameObject.Name} {b.Mass:0.0}kg {b.Velocity.Length:0} vs {predicted:0}";
			} ) ) + $" (dt {Time.Delta:0.0000})" );
		}

		if ( _last < 1 ) return;
		_last = 0;
		_ticks++;

		var players = Scene.GetAllComponents<PlayerController>()
			.Select( p => $"{p.GameObject.Name} at {p.WorldPosition.x:0},{p.WorldPosition.y:0},{p.WorldPosition.z:0} speed {p.Velocity.Length:0} {(p.IsOnGround ? $"on {p.GroundObject?.Name}" : "in the air")}" );
		var bodies = Scene.GetAllComponents<Rigidbody>().Where( b => b.PhysicsBody.IsValid() ).ToList();
		var awake = bodies.Count( b => !b.PhysicsBody.Sleeping );
		var restless = awake is > 0 and <= 3
			? " (" + string.Join( ", ", bodies.Where( b => !b.PhysicsBody.Sleeping ).Select( b => $"{b.GameObject.Name} at {b.Velocity.Length:0.0}" ) ) + ")"
			: "";
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
			// a model's collision names its own surfaces
			foreach ( var m in Scene.GetAllComponents<ModelCollider>() )
			{
				var surfaces = m.Model?.Physics?.Surfaces?.Select( s => s?.ResourceName ?? "none" ) ?? Enumerable.Empty<string>();
				var shapes = m.Rigidbody?.PhysicsBody?.Shapes ?? Enumerable.Empty<PhysicsShape>();
				Log.Info( $"shimwatch: {m.GameObject.Name} ({m.Model?.ResourceName}) surfaces [{string.Join( " ", surfaces )}], its shapes [{string.Join( " ", shapes.Select( s => s.Surface?.ResourceName ?? "none" ).Distinct() )}], mass {m.Rigidbody?.Mass:0.0}" );
			}
			// a map's ground, if there's a map: straight down at a few places, and what's there
			foreach ( var m in Scene.GetAllComponents<MapInstance>() )
			{
				var spawns = Scene.GetAllComponents<SpawnPoint>().Select( x => x.WorldPosition ).ToList();
				var shots = string.Join( ", ", new[] { Vector3.Zero, new Vector3( 1000, 1000, 0 ), new Vector3( -2000, 500, 0 ) }.Concat( spawns.Take( 2 ) ).Select( at =>
				{
					var tr = Scene.Trace.Ray( at.WithZ( 4000 ), at.WithZ( -4000 ) ).Run();
					return tr.Hit ? $"({at.x:0},{at.y:0}) z {tr.EndPosition.z:0} {tr.GameObject?.Name} {tr.Surface?.ResourceName} [{string.Join( " ", tr.Tags )}]" : $"({at.x:0},{at.y:0}) nothing";
				} ) );
				Log.Info( $"shimwatch: map {m.MapName} loaded {m.IsLoaded}, {spawns.Count} spawn points, map objects {Scene.GetAllComponents<MapObjectComponent>().Count()}; ground: {shots}" );
			}
			// and its key values: a prop's health, what it breaks into
			foreach ( var p in Scene.GetAllComponents<Prop>() )
				Log.Info( $"shimwatch: prop {p.GameObject.Name} health {p.Health:0} explosive {p.IsExplosive} gibs {p.Model?.GetData<Sandbox.ModelEditor.Nodes.ModelBreakPiece[]>()?.Length ?? 0}" );
		}

		// once: break the first prop that has gibs, and follow them
		if ( _ticks == 20 )
		{
			_broken = Scene.GetAllComponents<Prop>().FirstOrDefault( p => p.Model?.GetData<Sandbox.ModelEditor.Nodes.ModelBreakPiece[]>()?.Length > 0 && !p.IsExplosive );
			if ( _broken is not null )
			{
				Log.Info( $"shimwatch: breaking {_broken.GameObject.Name} ({_broken.Model.ResourceName}) at {_broken.WorldPosition}" );
				_broken.Kill();
			}
		}
		// once: set off the explosive prop, and see what it does to its neighbours
		if ( _ticks == 30 )
		{
			_bomb = Scene.GetAllComponents<Prop>().FirstOrDefault( p => p.IsExplosive );
			if ( _bomb is not null )
			{
				_blast = _bomb.WorldPosition;
				_near = Scene.GetAllComponents<Rigidbody>().Where( b => b.GameObject != _bomb.GameObject && b.WorldPosition.Distance( _blast ) < 300 )
					.ToDictionary( b => b, b => b.WorldPosition );
				Log.Info( $"shimwatch: detonating {_bomb.GameObject.Name} at {_blast}; within 300: {string.Join( ", ", _near.Keys.Select( b => $"{b.GameObject.Name} {b.WorldPosition.Distance( _blast ):0} away" + (b.Components.Get<Prop>() is { } p ? $" health {p.Health:0}" : "") ) )}" );
				_bomb.Kill();
				_blew = true;
			}
		}
		if ( _ticks is > 30 and <= 33 && _near is not null )
		{
			var gibs = Scene.GetAllComponents<Gib>().Count( g => g.WorldPosition.Distance( _blast ) < 400 );
			Log.Info( $"shimwatch blast {_ticks - 30}: {gibs} gibs near it, explosion {(Scene.GetAllComponents<RadiusDamage>().Any() ? "there" : "none")} | "
				+ string.Join( ", ", _near.Select( kv => !kv.Key.IsValid() ? $"(broken)" : $"{kv.Key.GameObject.Name} moved {kv.Key.WorldPosition.Distance( kv.Value ):0}" + (kv.Key.Components.Get<Prop>() is { } p ? $" health {p.Health:0}" : "") ) ) );
		}

		if ( _ticks > 20 && _ticks <= 26 && _broken is not null )
		{
			var gibs = Scene.GetAllComponents<Gib>().ToList();
			Log.Info( $"shimwatch gibs: {gibs.Count} ({string.Join( ", ", gibs.Select( g => $"{g.Model?.ResourceName} z {g.WorldPosition.z:0} {(g.Components.Get<Rigidbody>()?.PhysicsBody?.Sleeping == true ? "asleep" : "moving")}" ) )}) | {(_broken.IsValid() ? "the prop is still there" : "the prop is gone")}" );
		}

		Shoot();

		// each player: what the controller's ground probe finds under them, and the map still there
		foreach ( var p in Scene.GetAllComponents<PlayerController>() )
		{
			var probe = Scene.Trace.Box( new BBox( new Vector3( -16, -16, 0 ), new Vector3( 16, 16, 0 ) ), p.WorldPosition + Vector3.Up * 4, p.WorldPosition - Vector3.Up * 2 )
				.IgnoreGameObjectHierarchy( p.GameObject ).WithCollisionRules( p.Tags ).UseHitPosition().Run();
			var below = Scene.Trace.Ray( p.WorldPosition + Vector3.Up * 10, p.WorldPosition - Vector3.Up * 100000 ).IgnoreGameObjectHierarchy( p.GameObject ).Run();
			var map = Scene.GetAllComponents<MapCollider>().FirstOrDefault();
			Log.Info( $"shimwatch player {p.GameObject.Name}: tags [{string.Join( " ", p.Tags )}] probe {(probe.Hit ? $"{probe.GameObject?.Name} frac {probe.Fraction:0.00} solid {probe.StartedSolid}" : "nothing")} | below {(below.Hit ? $"{below.GameObject?.Name} z {below.EndPosition.z:0}" : "nothing")} | map collider {(map is null ? "gone" : $"{(map.Enabled ? "on" : "off")} at {map.WorldPosition}")} | body {(p.Body is { } b ? $"motion {b.MotionEnabled} gravity {b.Gravity} v {b.Velocity.z:0}" : "none")}" );
		}

		Log.Info( $"shimwatch {_ticks}: players [{string.Join( "; ", players )}] | bodies {bodies.Count} awake {awake}{restless} z {zs.Min():0}..{zs.Max():0} | joints {joints} | colliders {colliders}" );
	}

	const string TargetModel = "models/first_person/v_first_person_arms_human.vmdl";
	SkinnedModelRenderer _target;
	Vector3 _rest;

	void Shoot()
	{
		if ( _ticks == 4 )
		{
			var go = new GameObject( true, "shimtarget" );
			go.WorldPosition = new Vector3( 0, 2000, 500 );
			_target = go.AddComponent<SkinnedModelRenderer>();
			_target.Model = Model.Load( TargetModel );
			var hitboxes = go.AddComponent<ModelHitboxes>();
			hitboxes.Renderer = _target;
			var m = _target.Model;
			Log.Info( $"shimwatch target: {m?.Name} error {m?.IsError} bones {m?.BoneCount} meshes {m?.MeshCount} hitboxes {m?.HitboxSet?.All?.Count} attachments {m?.Attachments?.Count} animations {m?.AnimationCount} ({string.Join( ", ", m?.AnimationNames?.Take( 3 ) ?? [] )}..) bounds {m?.Bounds.Size}" );
		}

		// turned a quarter about z, and moved, halfway through: the hitboxes follow
		if ( _ticks == 9 && _target.IsValid() )
		{
			_target.WorldRotation = Rotation.FromYaw( 90 );
			_target.WorldPosition = new Vector3( 300, 2000, 600 );
		}

		// a punch, looping: the bones move, and their hitboxes with them
		if ( _ticks == 13 && _target.IsValid() )
		{
			if ( _target.TryGetBoneTransform( "hand_R", out var rest ) ) _rest = rest.Position;
			_target.UseAnimGraph = false;
			_target.Sequence.Name = "Punching_Right_Hit_01";
			_target.Sequence.Looping = true;
			Log.Info( $"shimwatch playing {_target.Sequence.Name} for {_target.Sequence.Duration:0.00}s" );
		}

		if ( _ticks > 13 && _ticks <= 17 && _target.IsValid() && _target.TryGetBoneTransform( "hand_R", out var now ) )
		{
			// where the hand rested: its hitbox has gone with it
			var tr = Scene.Trace.Ray( _rest + Vector3.Forward * 200, _rest - Vector3.Forward * 50 ).UseHitboxes().Run();
			Log.Info( $"shimwatch punch at {_target.Sequence.TimeNormalized:0.00}: hand_R {(now.Position - _rest).Length:0.0} from rest | where it rested: {(tr.Hit ? $"hit {tr.Hitbox?.Bone?.Name}" : "missed")}" );
		}

		if ( _ticks < 5 || _ticks > 17 || !_target.IsValid() ) return;

		foreach ( var name in new[] { "hand_R", "arm_upper_L", "finger_index_1_R" } )
		{
			var bone = _target.Model.Bones.GetBone( name );
			if ( bone is null || !_target.TryGetBoneTransform( bone, out var tx ) )
			{
				Log.Info( $"shimwatch shot {name}: no bone transform" );
				continue;
			}

			// from 200 away along x, at the bone
			var from = tx.Position + Vector3.Forward * 200;
			var tr = Scene.Trace.Ray( from, tx.Position - Vector3.Forward * 50 ).UseHitboxes().Run();
			var miss = Scene.Trace.Ray( from + Vector3.Up * 300, tx.Position + Vector3.Up * 300 - Vector3.Forward * 50 ).UseHitboxes().Run();
			Log.Info( $"shimwatch shot {name} at {tx.Position.x:0.0},{tx.Position.y:0.0},{tx.Position.z:0.0}: {(tr.Hit ? $"hit {tr.Hitbox?.Bone?.Name} on {tr.GameObject?.Name} at {tr.HitPosition.x:0.0},{tr.HitPosition.y:0.0},{tr.HitPosition.z:0.0} after {tr.Distance:0.0}" : "missed")} | above it: {(miss.Hit ? "hit" : "missed")}" );
		}
	}
}
