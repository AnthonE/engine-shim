/// <summary>
/// Says once a second what the game's world is doing, for a server with no
/// screen: each player (where, standing on what), the bodies (how many,
/// how many awake, how high), the joints. And shoots at a model's
/// hitboxes: a skinned model from s&box's download cache, with
/// ModelHitboxes, hit by traces aimed at its bones, then turned and moved,
/// then playing a sequence.
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

		Shoot();

		Log.Info( $"shimwatch {_ticks}: players [{string.Join( "; ", players )}] | bodies {bodies.Count} awake {awake} z {zs.Min():0}..{zs.Max():0} | joints {joints} | colliders {colliders}" );
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
