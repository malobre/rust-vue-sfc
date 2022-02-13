use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use pprof::criterion::{Output, PProfProfiler};

const VUE_SFC: &str = r#"<script>
export default {
  data() {
    return {
      greeting: 'Hello World!'
    }
  }
}
</script>

<template>
  <p class="greeting">{{ greeting }}</p>
</template>

<style>
.greeting {
  color: red;
  font-weight: bold;
}
</style>"#;

pub fn parse_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse");
    group.throughput(Throughput::Bytes(u64::try_from(VUE_SFC.len()).unwrap()));
    group.bench_with_input(
        BenchmarkId::from_parameter(VUE_SFC.len()),
        VUE_SFC,
        |b, input| {
            b.iter(|| vue_sfc::parse(input).unwrap());
        },
    );
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(1000, Output::Flamegraph(None)));
    targets = parse_benchmark
}

criterion_main!(benches);
