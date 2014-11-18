use std::os;
use std::io::File;

static C_PREFIX: &'static str = "C";
static T_PREFIX: &'static str = "T";

fn main() {
    let dst = Path::new(os::getenv("OUT_DIR").expect("build script run outside of Cargo"));
    let f = File::create(&dst.join("impls.rs")).ok().expect("could not create out file");

    generate(f, 32);
}

fn generate(mut file: File, limit: uint) {
    for i in range(1, limit) {
        (write!(file,
"impl<{ts},{cs}> Reduce<T1, T{n}> for ({cs})
where {bounds} {{
    fn reduce(self, a: T1) -> T{n} {{
        {applications}
        return a;
    }}
}}\n\n",
        ts = types(T_PREFIX, i + 2).connect(","),
        cs = types(C_PREFIX, i + 1).connect(","),
        n = i + 2,
        bounds = bounds(types(C_PREFIX, i + 1), fn_once_closure).connect(",\n      "),
        applications = applications(i, "call_once((a,))").connect("\n        "),
        )).unwrap()
    }

    for i in range(1, limit) {
        (write!(file,
"impl<'a, {ts},{cs}> Reduce<T1, T{n}> for &'a ({cs})
where {bounds} {{
    fn reduce(self, a: T1) -> T{n} {{
        {applications}
        return a;
    }}
}}\n\n",
        ts = types(T_PREFIX, i + 2).connect(","),
        cs = types(C_PREFIX, i + 1).connect(","),
        n = i + 2,
        bounds = bounds(types(C_PREFIX, i + 1), fn_closure).connect(",\n      "),
        applications = applications(i, "call((a,))").connect("\n        "),
        )).unwrap()
    }

    for i in range(1, limit) {
        (write!(file,
"impl<'a, {ts},{cs}> Reduce<T1, T{n}> for &'a mut ({cs})
where {bounds} {{
    fn reduce(self, a: T1) -> T{n} {{
        {applications}
        return a;
    }}
}}\n\n",
        ts = types(T_PREFIX, i + 2).connect(","),
        cs = types(C_PREFIX, i + 1).connect(","),
        n = i + 2,
        bounds = bounds(types(C_PREFIX, i + 1), fn_mut_closure).connect(",\n      "),
        applications = applications(i, "call_mut((a,))").connect("\n        "),
        )).unwrap()
    }
}

fn types(prefix: &str, limit: uint) -> Vec<String> {
    range(1, limit + 1).map(|n| format!("{}{}", prefix, n)).collect()
}

fn bounds(cs: Vec<String>, trt: fn(uint, uint) -> String) -> Vec<String> {
    cs.into_iter()
        .enumerate()
        .map(|(i, c)| format!("{}: {}", c, trt(i + 1, i + 2)))
        .collect()
}

fn fn_closure(one: uint, two: uint) -> String {
    format!("Fn(T{}) -> T{}", one, two)
}

fn fn_once_closure(one: uint, two: uint) -> String {
    format!("FnOnce(T{}) -> T{}", one, two)
}

fn fn_mut_closure(one: uint, two: uint) -> String {
    format!("FnMut(T{}) -> T{}", one, two)
}

fn applications(limit: uint, op: &str) -> Vec<String> {
    range(0, limit + 1)
        .map(|n| format!("let a = self.{}.{};", n, op))
        .collect::<Vec<String>>()
}

