use std::marker::PhantomData;

use amethyst::{
    core::{
        bundle::SystemBundle,
        deferred_dispatcher_operation::{AddBundle, AddSystem, AddSystemDesc, DispatcherOperation},
        ArcThreadPool,
    },
    ecs::{Dispatcher, DispatcherBuilder, System},
    prelude::*,
    DataDispose, Error,
};

pub struct NiceGameData<'a, 'b> {
    dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> NiceGameData<'a, 'b> {
    pub fn new(dispatcher: Dispatcher<'a, 'b>) -> Self {
        NiceGameData {
            dispatcher: Some(dispatcher),
        }
    }
    pub fn update(&mut self, world: &World) {
        if let Some(dispatcher) = &mut self.dispatcher.as_mut() {
            dispatcher.dispatch(&world);
        }
    }

    pub fn dispose(&mut self, mut world: &mut World) {
        if let Some(dispatcher) = self.dispatcher.take() {
            dispatcher.dispose(&mut world);
        }
    }
}

// why is this neccessary if we impls in NiceGameData?
impl DataDispose for NiceGameData<'_, '_> {
    fn dispose(&mut self, world: &mut World) {
        self.dispose(world);
    }
}

pub struct NiceGameDataBuilder<'a, 'b> {
    dispatcher_ops: Vec<Box<dyn DispatcherOperation<'a, 'b>>>,
    dispatcher_builder: DispatcherBuilder<'a, 'b>,
}

impl<'a, 'b> Default for NiceGameDataBuilder<'a, 'b> {
    fn default() -> Self {
        NiceGameDataBuilder::new()
    }
}

impl<'a, 'b> NiceGameDataBuilder<'a, 'b> {
    pub fn new() -> Self {
        NiceGameDataBuilder {
            dispatcher_ops: Vec::new(),
            dispatcher_builder: DispatcherBuilder::new(),
        }
    }

    pub fn with_bundle<B>(mut self, bundle: B) -> Self
    where
        B: SystemBundle<'a, 'b> + 'static,
    {
        self.dispatcher_ops.push(Box::new(AddBundle { bundle }));
        self
    }

    pub fn with_system_desc<SD, S, N>(
        mut self,
        system_desc: SD,
        name: N,
        dependencies: &[N],
    ) -> Self
    where
        SD: SystemDesc<'a, 'b, S> + 'static,
        S: for<'c> System<'c> + 'static + Send,
        N: Into<String> + Clone,
    {
        let name = Into::<String>::into(name);
        let dependencies = dependencies
            .iter()
            .map(Clone::clone)
            .map(Into::<String>::into)
            .collect::<Vec<String>>();
        let op = Box::new(AddSystemDesc {
            system_desc,
            name,
            dependencies,
            marker: PhantomData::<S>,
        }) as Box<dyn DispatcherOperation<'a, 'b> + 'static>;
        self.dispatcher_ops.push(op);
        self
    }

    pub fn with<S, N>(mut self, system: S, name: N, dependencies: &[N]) -> Self
    where
        S: for<'c> System<'c> + 'static + Send,
        N: Into<String> + Clone,
    {
        let name = Into::<String>::into(name);
        let dependencies = dependencies
            .iter()
            .map(Clone::clone)
            .map(Into::<String>::into)
            .collect::<Vec<String>>();

        let op = Box::new(AddSystem {
            system,
            name,
            dependencies,
        }) as Box<dyn DispatcherOperation<'a, 'b> + 'static>;
        self.dispatcher_ops.push(op);
        self
    }

    //TODO config values for no threading
    pub fn build_dispatcher(self, mut world: &mut World) -> Dispatcher<'a, 'b> {
        let pool = (*world.read_resource::<ArcThreadPool>()).clone();
        let mut dispatcher_builder = self.dispatcher_builder;
        self.dispatcher_ops
            .into_iter()
            .try_for_each(|dispatcher_op| dispatcher_op.exec(world, &mut dispatcher_builder))
            .unwrap_or_else(|e| panic!("Unable to init dispatcher: {}", e));

        let mut dispatcher = dispatcher_builder.with_pool(pool).build();
        dispatcher.setup(&mut world);
        dispatcher
    }
}

impl<'a, 'b> DataInit<NiceGameData<'a, 'b>> for NiceGameDataBuilder<'a, 'b> {
    fn build(self, world: &mut World) -> NiceGameData<'a, 'b> {
        NiceGameData::new(self.build_dispatcher(world))
    }
}
