#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum ScheduleLabel {
    #[default]
    Main,

    /// The schedule that runs before [`Startup`].
    ///
    /// See the [`Main`] schedule for some details about how schedules are run.
    PreStartup,
    /// The schedule that runs once when the app starts.
    ///
    /// See the [`Main`] schedule for some details about how schedules are run.
    Startup,
    /// The schedule that runs once after [`Startup`].
    ///
    /// See the [`Main`] schedule for some details about how schedules are run.
    PostStartup,
    /// Runs first in the schedule.
    ///
    /// See the [`Main`] schedule for some details about how schedules are run.
    First,
    /// The schedule that contains logic that must run before [`Update`]. For example, a system that reads raw keyboard
    /// input OS events into a `Messages` resource. This enables systems in [`Update`] to consume the messages from the `Messages`
    /// resource without actually knowing about (or taking a direct scheduler dependency on) the "os-level keyboard event system".
    ///
    /// [`PreUpdate`] exists to do "engine/plugin preparation work" that ensures the APIs consumed in [`Update`] are "ready".
    /// [`PreUpdate`] abstracts out "pre work implementation details".
    ///
    /// See the [`Main`] schedule for some details about how schedules are run.
    PreUpdate,
    /// Runs the [`FixedMain`] schedule in a loop according until all relevant elapsed time has been "consumed".
    ///
    /// If you need to order your variable timestep systems before or after
    /// the fixed update logic, use the [`RunFixedMainLoopSystems`] system set.
    ///
    /// Note that in contrast to most other Bevy schedules, systems added directly to
    /// [`RunFixedMainLoop`] will *not* be parallelized between each other.
    ///
    /// See the [`Main`] schedule for some details about how schedules are run.
    RunFixedMainLoop,
    /// Runs first in the [`FixedMain`] schedule.
    ///
    /// See the [`FixedMain`] schedule for details on how fixed updates work.
    /// See the [`Main`] schedule for some details about how schedules are run.
    FixedFirst,
    /// The schedule that contains logic that must run before [`FixedUpdate`].
    ///
    /// See the [`FixedMain`] schedule for details on how fixed updates work.
    /// See the [`Main`] schedule for some details about how schedules are run.
    FixedPreUpdate,
    /// The schedule that contains most gameplay logic, which runs at a fixed rate rather than every render frame.
    /// For logic that should run once per render frame, use the [`Update`] schedule instead.
    ///
    /// Examples of systems that should run at a fixed rate include (but are not limited to):
    /// - Physics
    /// - AI
    /// - Networking
    /// - Game rules
    ///
    /// See the [`Update`] schedule for examples of systems that *should not* use this schedule.
    /// See the [`FixedMain`] schedule for details on how fixed updates work.
    /// See the [`Main`] schedule for some details about how schedules are run.
    FixedUpdate,
    /// The schedule that runs after the [`FixedUpdate`] schedule, for reacting
    /// to changes made in the main update logic.
    ///
    /// See the [`FixedMain`] schedule for details on how fixed updates work.
    /// See the [`Main`] schedule for some details about how schedules are run.
    FixedPostUpdate,
    /// The schedule that runs last in [`FixedMain`]
    ///
    /// See the [`FixedMain`] schedule for details on how fixed updates work.
    /// See the [`Main`] schedule for some details about how schedules are run.
    FixedLast,
    /// The schedule that contains systems which only run after a fixed period of time has elapsed.
    ///
    /// This is run by the [`RunFixedMainLoop`] schedule. If you need to order your variable timestep systems
    /// before or after the fixed update logic, use the [`RunFixedMainLoopSystems`] system set.
    FixedMain,
    /// The schedule that contains any app logic that must run once per render frame.
    /// For most gameplay logic, consider using [`FixedUpdate`] instead.
    ///
    /// Examples of systems that should run once per render frame include (but are not limited to):
    /// - UI
    /// - Input handling
    /// - Audio control
    ///
    /// See the [`FixedUpdate`] schedule for examples of systems that *should not* use this schedule.
    /// See the [`Main`] schedule for some details about how schedules are run.
    Update,
    /// The schedule that contains scene spawning.
    ///
    /// This runs after [`Update`] and before [`PostUpdate`]. See the [`Main`] schedule for more details about how schedules are run.
    SpawnScene,
    /// The schedule that contains logic that must run after [`Update`]. For example, synchronizing "local transforms" in a hierarchy
    /// to "global" absolute transforms. This enables the [`PostUpdate`] transform-sync system to react to "local transform" changes in
    /// [`Update`] without the [`Update`] systems needing to know about (or add scheduler dependencies for) the "global transform sync system".
    ///
    /// [`PostUpdate`] exists to do "engine/plugin response work" to things that happened in [`Update`].
    /// [`PostUpdate`] abstracts out "implementation details" from users defining systems in [`Update`].
    ///
    /// See the [`Main`] schedule for some details about how schedules are run.
    PostUpdate,
    /// Runs last in the schedule.
    ///
    /// See the [`Main`] schedule for some details about how schedules are run.
    Last,
    /// Animation system set. This exists in [`PostUpdate`].
    AnimationSystems,
}

impl ScheduleLabel {
    pub const ALL: [ScheduleLabel; 18] = [
        ScheduleLabel::Main,
        ScheduleLabel::PreStartup,
        ScheduleLabel::Startup,
        ScheduleLabel::PostStartup,
        ScheduleLabel::First,
        ScheduleLabel::PreUpdate,
        ScheduleLabel::RunFixedMainLoop,
        ScheduleLabel::FixedFirst,
        ScheduleLabel::FixedPreUpdate,
        ScheduleLabel::FixedUpdate,
        ScheduleLabel::FixedPostUpdate,
        ScheduleLabel::FixedLast,
        ScheduleLabel::FixedMain,
        ScheduleLabel::Update,
        ScheduleLabel::SpawnScene,
        ScheduleLabel::PostUpdate,
        ScheduleLabel::Last,
        ScheduleLabel::AnimationSystems,
    ];
    pub const PER_FRAME: [ScheduleLabel; 9] = [
        ScheduleLabel::PreUpdate,
        ScheduleLabel::RunFixedMainLoop,
        ScheduleLabel::FixedFirst,
        ScheduleLabel::FixedPreUpdate,
        ScheduleLabel::FixedUpdate,
        ScheduleLabel::FixedPostUpdate,
        ScheduleLabel::Update,
        ScheduleLabel::PostUpdate,
        ScheduleLabel::AnimationSystems,
    ];

    pub const START_UP: [ScheduleLabel; 5] = [
        ScheduleLabel::PreStartup,
        ScheduleLabel::Startup,
        ScheduleLabel::PostStartup,
        ScheduleLabel::First,
        ScheduleLabel::SpawnScene,
    ];

    pub const LAST: [ScheduleLabel; 2] = [ScheduleLabel::FixedLast, ScheduleLabel::Last];
}
