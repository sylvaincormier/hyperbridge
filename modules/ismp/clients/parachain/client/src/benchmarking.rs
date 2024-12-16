#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use ismp::host::StateMachine;

#[benchmarks(where T: Config)]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn add_parachain() -> Result<(), BenchmarkError> {
        let state_machines: Vec<ParachainData> = (0..10)
            .map(|i| ParachainData {
                id: i as u32,
                slot_duration: 6000,
            })
            .collect();

        #[block]
        {
            Pallet::<T>::add_parachain(RawOrigin::Root.into(), state_machines)?;
        }

        Ok(())
    }

    #[benchmark]
    fn remove_parachain() -> Result<(), BenchmarkError> {
        let state_machines: Vec<u32> = (0..10).map(|i| i as u32).collect();
        
        for i in 0..10 {
            Parachains::<T>::insert(i as u32, 6000);
        }

        #[block]
        {
            Pallet::<T>::remove_parachain(RawOrigin::Root.into(), state_machines)?;
        }

        Ok(())
    }
}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);