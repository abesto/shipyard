use core::any::type_name;
use shipyard::error;
use shipyard::*;

#[test]
fn unique_storage() {
    let world = World::default();
    world.try_add_unique(0usize).unwrap();

    world
        .try_run(|mut x: UniqueViewMut<usize>| {
            *x += 1;
        })
        .unwrap();
    world
        .try_run(|x: UniqueView<usize>| {
            assert_eq!(*x, 1);
        })
        .unwrap();
}

#[test]
fn not_unique_storage() {
    let world = World::new();

    match world.try_run(|_: UniqueView<usize>| {}).err() {
        Some(error::Run::GetStorage(get_storage)) => assert_eq!(
            get_storage,
            error::GetStorage::MissingUnique(type_name::<usize>())
        ),
        _ => panic!(),
    }

    match world.try_run(|_: UniqueViewMut<usize>| {}).err() {
        Some(error::Run::GetStorage(get_storage)) => assert_eq!(
            get_storage,
            error::GetStorage::MissingUnique(type_name::<usize>())
        ),
        _ => panic!(),
    }
}

#[cfg(feature = "non_send")]
#[test]
fn non_send() {
    struct NonSendStruct {
        value: usize,
        _phantom: core::marker::PhantomData<*const ()>,
    }
    unsafe impl Sync for NonSendStruct {}

    let world = World::default();
    world
        .try_add_unique_non_send(NonSendStruct {
            value: 0,
            _phantom: core::marker::PhantomData,
        })
        .unwrap();

    world
        .try_run(|mut x: NonSend<UniqueViewMut<NonSendStruct>>| {
            x.value += 1;
        })
        .unwrap();
    world
        .try_run(|x: NonSend<UniqueView<NonSendStruct>>| {
            assert_eq!(x.value, 1);
        })
        .unwrap();
}

#[cfg(feature = "non_sync")]
#[test]
fn non_sync() {
    struct NonSyncStruct {
        value: usize,
        _phantom: core::marker::PhantomData<*const ()>,
    }
    unsafe impl Send for NonSyncStruct {}

    let world = World::default();
    world
        .try_add_unique_non_sync(NonSyncStruct {
            value: 0,
            _phantom: core::marker::PhantomData,
        })
        .unwrap();

    world
        .try_run(|mut x: NonSync<UniqueViewMut<NonSyncStruct>>| {
            x.value += 1;
        })
        .unwrap();
    world
        .try_run(|x: NonSync<UniqueView<NonSyncStruct>>| {
            assert_eq!(x.value, 1);
        })
        .unwrap();
}

#[cfg(all(feature = "non_send", feature = "non_sync"))]
#[test]
fn non_send_sync() {
    struct NonSendSyncStruct {
        value: usize,
        _phantom: core::marker::PhantomData<*const ()>,
    }

    let world = World::default();
    world
        .try_add_unique_non_send_sync(NonSendSyncStruct {
            value: 0,
            _phantom: core::marker::PhantomData,
        })
        .unwrap();

    world
        .try_run(|mut x: NonSendSync<UniqueViewMut<NonSendSyncStruct>>| {
            x.value += 1;
        })
        .unwrap();
    world
        .try_run(|x: NonSendSync<UniqueView<NonSendSyncStruct>>| {
            assert_eq!(x.value, 1);
        })
        .unwrap();
}
