# Template

为 template 模块的 do_something 添加 benchmark 用例（也可以是其它自选模块的可调用函数），并且将 benchmark 运行的结果转换为对应的权重定义；
选择 node-template 或者其它节点程序，生成 Chain Spec 文件（两种格式都需要）；
（附加题）根据 Chain Spec，部署公开测试网络

## 添加`benchmark`用例

[benchmarking.rs](./src/benchmarking.rs)

```rust

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	do_something {
		let s in 0 .. 1000;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), s)
	verify {
		assert_eq!(Something::<T>::get(), Some(s));
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}

```

[weights.rs](./src/weights.rs)

```rust

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn do_something() -> Weight;
}

/// Weight functions for `pallet_template`.
pub struct TemplateWeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for TemplateWeightInfo<T> {
	// Storage: TemplateModule Something (r:0 w:1)
	fn do_something() -> Weight {
		(18_945_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

```

## 生成`Chain Spec`文件（两种格式都需要）

[local.json](../../specs/local.json)

[local_raw.json](../../specs/local_raw.json)