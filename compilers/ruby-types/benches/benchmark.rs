use criterion::{Criterion, criterion_group, criterion_main};
use ruby_types::RubyValue;

fn bench_array_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("array_operations");

    // Test push
    group.bench_function("push", |b| {
        let mut arr = RubyValue::Array(vec![]);
        b.iter(|| {
            for i in 0..1000 {
                arr.push(RubyValue::Integer(i as i32));
            }
        });
    });

    // Test pop
    group.bench_function("pop", |b| {
        let mut arr = RubyValue::Array((0..1000).map(|i| RubyValue::Integer(i as i32)).collect());
        b.iter(|| {
            for _ in 0..1000 {
                arr.pop();
            }
        });
    });

    // Test shift
    group.bench_function("shift", |b| {
        let mut arr = RubyValue::Array((0..1000).map(|i| RubyValue::Integer(i as i32)).collect());
        b.iter(|| {
            for _ in 0..1000 {
                arr.shift();
            }
        });
    });

    // Test unshift
    group.bench_function("unshift", |b| {
        let mut arr = RubyValue::Array(vec![]);
        b.iter(|| {
            for i in 0..1000 {
                arr.unshift(RubyValue::Integer(i as i32));
            }
        });
    });

    // Test array_get
    group.bench_function("array_get", |b| {
        let arr = RubyValue::Array((0..1000).map(|i| RubyValue::Integer(i as i32)).collect());
        b.iter(|| {
            for i in 0..999 {
                arr.array_get(i);
            }
        });
    });

    // Test array_set
    group.bench_function("array_set", |b| {
        let mut arr = RubyValue::Array((0..1000).map(|i| RubyValue::Integer(i as i32)).collect());
        b.iter(|| {
            for i in 0..999 {
                arr.array_set(i, RubyValue::Integer((i * 2) as i32));
            }
        });
    });

    group.finish();
}

fn bench_hash_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash_operations");

    // Test hash_set
    group.bench_function("hash_set", |b| {
        let mut hash = RubyValue::Hash(std::collections::HashMap::new());
        b.iter(|| {
            for i in 0..1000 {
                hash.hash_set(&format!("key_{}", i), RubyValue::Integer(i as i32));
            }
        });
    });

    // Test hash_get
    group.bench_function("hash_get", |b| {
        let mut hash = RubyValue::Hash(std::collections::HashMap::new());
        for i in 0..1000 {
            hash.hash_set(&format!("key_{}", i), RubyValue::Integer(i as i32));
        }
        b.iter(|| {
            for i in 0..999 {
                hash.hash_get(&format!("key_{}", i));
            }
        });
    });

    // Test merge
    group.bench_function("merge", |b| {
        let mut hash1 = RubyValue::Hash(std::collections::HashMap::new());
        for i in 0..500 {
            hash1.hash_set(&format!("key_{}", i), RubyValue::Integer(i as i32));
        }
        let mut hash2 = RubyValue::Hash(std::collections::HashMap::new());
        for i in 500..1000 {
            hash2.hash_set(&format!("key_{}", i), RubyValue::Integer(i as i32));
        }
        b.iter(|| {
            hash1.merge(&hash2);
        });
    });

    // Test select
    group.bench_function("select", |b| {
        let mut hash = RubyValue::Hash(std::collections::HashMap::new());
        for i in 0..1000 {
            hash.hash_set(&format!("key_{}", i), RubyValue::Integer(i as i32));
        }
        b.iter(|| {
            hash.select(|_, v| v.to_i32() % 2 == 0);
        });
    });

    group.finish();
}

fn bench_string_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_operations");

    // Test to_string
    group.bench_function("to_string", |b| {
        let values = vec![
            RubyValue::Nil,
            RubyValue::Boolean(true),
            RubyValue::Integer(42),
            RubyValue::Float(3.14),
            RubyValue::String("hello".to_string()),
            RubyValue::Symbol("symbol".to_string()),
        ];
        b.iter(|| {
            for value in &values {
                value.to_string();
            }
        });
    });

    // Test concat
    group.bench_function("concat", |b| {
        let s1 = RubyValue::String("hello".repeat(100));
        let s2 = RubyValue::String("world".repeat(100));
        b.iter(|| {
            s1.concat(&s2);
        });
    });

    // Test split
    group.bench_function("split", |b| {
        let s = RubyValue::String("a,b,c,d,e,f,g,h,i,j".repeat(100));
        b.iter(|| {
            s.split(",");
        });
    });

    // Test upcase/downcase
    group.bench_function("upcase", |b| {
        let s = RubyValue::String("hello world".repeat(100));
        b.iter(|| {
            s.upcase();
        });
    });

    group.bench_function("downcase", |b| {
        let s = RubyValue::String("HELLO WORLD".repeat(100));
        b.iter(|| {
            s.downcase();
        });
    });

    group.finish();
}

fn bench_numeric_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("numeric_operations");

    // Test to_i32
    group.bench_function("to_i32", |b| {
        let values = vec![RubyValue::Integer(42), RubyValue::Float(3.14), RubyValue::Boolean(true)];
        b.iter(|| {
            for value in &values {
                value.to_i32();
            }
        });
    });

    // Test to_f64
    group.bench_function("to_f64", |b| {
        let values = vec![RubyValue::Integer(42), RubyValue::Float(3.14), RubyValue::Boolean(true)];
        b.iter(|| {
            for value in &values {
                value.to_f64();
            }
        });
    });

    // Test abs
    group.bench_function("abs", |b| {
        let values = vec![RubyValue::Integer(-42), RubyValue::Float(-3.14)];
        b.iter(|| {
            for value in &values {
                value.abs();
            }
        });
    });

    group.finish();
}

criterion_group!(benches, bench_array_operations, bench_hash_operations, bench_string_operations, bench_numeric_operations);
criterion_main!(benches);
