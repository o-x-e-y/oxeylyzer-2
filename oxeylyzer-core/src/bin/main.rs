fn main() {
    println!("waddahell")
}

// #[allow(unused_imports)]
// use oxeylyzer_core::{
//     analyze::Analyzer,
//     corpus_cleaner::CorpusCleaner,
//     data::Data,
//     layout::{Layout, PosPair},
//     weights::Weights,
//     REPLACEMENT_CHAR,
// };
// use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

// fn main() {
//     let name = "italian";

//     let cleaner = CorpusCleaner::builder()
//         .with_chars("abcdefghijklmnopqrstuvwxyz".chars())
//         .qwerty_punctuation_uppercase(true)
//         .with_chars(['\t', '\n'])
//         .with_dead_key(
//             [
//                 ('à', 'a'),
//                 ('è', 'e'),
//                 ('ì', 'i'),
//                 ('ò', 'o'),
//                 ('ù', 'u'),
//                 // ('À', 'A'), ('È', 'E'), ('Ì', 'I'), ('Ò', 'O'), ('Ù', 'U')
//             ],
//             '*',
//         )
//         // .shift_char(Some(REPLACEMENT_CHAR))
//         .build();
//     // let cleaner = CorpusCleaner::raw();

//     let data = time_this::time!(Data::from_path(format!("./corpora/{name}"), name, &cleaner)
//         .expect("this path should exist"));

//     data.save("./data")
//         .expect("the folder exists, surely this can't go wrong");

//     let data = Data::load(format!("./data/{name}.json")).expect("this should exist");

//     let weights = Weights {
//         heatmap: -1,
//         sfbs: -3000,
//         sfs: -500,
//     };

//     let analyzer = Analyzer::new(data, weights);

//     let layout =
//         Layout::load("./layouts/heatmap1.dof").expect("this layout is valid and exists, soooo");

//     // let (layout, score) = analyzer.greedy_depth2_improve(layout);

//     // println!("custom improved layout:\nscore: {score}");
//     // analyzer.show_stats(&layout);

//     let layout = layout.random();

//     // let (layout, score) = time_this::time!(analyzer.optimize_depth(layout));

//     // println!("optimize depth layout:\nscore: {score}");
//     // analyzer.show_stats(&layout);

//     let (layout, score) = time_this::time!(analyzer.always_better_swap(layout));

//     println!("optimize alwaps_best_swap:\nscore: {score}");
//     analyzer.show_stats(&layout);

//     let mut v = [0, 1, 4, 3];

//     v.sort_unstable_by(|s1, s2| s2.cmp(s1));

//     println!("{v:?}");

//     // let (layout, score) = time_this::time!(analyzer.annealing_improve(layout, 400.0, 0.95, 3000));

//     // println!("annealing layout:\nscore: {score}");
//     // analyzer.show_stats(&layout);

//     // let (layout, score2) = analyzer.greedy_improve(layout);

//     // println!("greedy continued layout:\nscore: {score}");
//     // analyzer.show_stats(&layout);

//     // let layout2 =
//     //     Layout::load("./layouts/heatmap2.dof").expect("this layout is valid and exists, soooo");
//     // let dh =
//     //     Layout::load("./layouts/colemak-dh.dof").expect("this layout is valid and exists, soooo");

//     // println!("layout1:");
//     // analyzer.stats(&layout);

//     // println!("\nlayout2:");
//     // analyzer.stats(&layout2);

//     // println!("metric contributions:");
//     // analyzer.metric_contribution(&layout2);

//     // let mut cache = analyzer.cached_layout(layout2);
//     // let (swap, score) = analyzer.best_swap(&mut cache).unwrap();

//     // println!("\nswap: {swap:?}");
//     // println!("upgraded layout2:\nscore: {score}");
//     // cache.swap(swap);
//     // analyzer.stats(&cache.clone().into());

//     // println!("\ncolemak dh:");
//     // analyzer.show_stats(&dh);

//     let mut layouts = Vec::with_capacity(1000);
//     time_this::time!((0..10)
//         .into_par_iter()
//         .map(|_| {
//             let starting_layout = layout.random();
//             analyzer.annealing_improve(starting_layout, 20_500_000_000_000.0, 0.983, 4000)
//         })
//         .collect_into_vec(&mut layouts));

//     layouts.sort_by(|(_, s1), (_, s2)| s2.cmp(s1));

//     println!("best layout:");
//     analyzer.show_stats(&layouts[0].0);

//     println!("metric contributions:");
//     analyzer.metric_contribution(&layouts[0].0);
// }

// #[test]
// fn thing() {
//     use std::io::Write;

//     fn best_of_1000(analyzer: &Analyzer, layout: Layout, temp: f64, coolf: f64, it: usize) -> i64 {
//         (0..1000)
//             .into_par_iter()
//             .map(|_| {
//                 let starting_layout = layout.random();
//                 analyzer
//                     .annealing_improve(starting_layout, temp, coolf, it)
//                     .1
//             })
//             .max()
//             .unwrap_or(i64::MIN)
//     }

//     let data = Data::load("./data/shai.json").expect("this should exist");

//     let weights = Weights {
//         heatmap: -1,
//         sfbs: -3000,
//         sfs: -500,
//     };

//     let analyzer = Analyzer::new(data, weights);

//     let layout =
//         Layout::load("./layouts/heatmap1.dof").expect("this layout is valid and exists, soooo");

//     let mut res = String::with_capacity(50 * 50 * 20 * 80);
//     res.push_str("temp,cooling_factor,iterations,best_score,time\n");

//     for _ in 0..10 {
//         let _ = best_of_1000(&analyzer, layout.clone(), 10.0, 0.9, 100);
//     }

//     for temp in (50..=500).step_by(50) {
//         let temp = temp as f64;

//         for coolf in (920..=980).step_by(10) {
//             let coolf = (coolf as f64) / 1000.0;

//             for it in (1000..=5000).step_by(1000) {
//                 let start = std::time::Instant::now();
//                 let best = best_of_1000(&analyzer, layout.clone(), temp, coolf, it);
//                 let end = std::time::Instant::now();
//                 let time = (end - start).as_micros();
//                 res.push_str(&format!("{temp},{coolf},{it},{best},{time}\n"));
//             }
//             // println!("cooling_factor: {coolf}, {}/{}", coolf/0.5, 0.995);
//         }
//         // println!("temp: {temp}, {}/{}", temp/10.0, 500.0/10.0);
//     }

//     let mut f = std::fs::OpenOptions::new()
//         .write(true)
//         .truncate(true)
//         .create(true)
//         .open("./annealing_data_250_2.csv")
//         .unwrap();

//     f.write_all(res.as_bytes()).unwrap();
// }

// #[test]
// fn thing2() {
//     use std::{fmt::Write as _, io::Write as _};

//     let data = Data::load("./data/shai.json").expect("this should exist");

//     let weights = Weights {
//         heatmap: -1,
//         sfbs: -3000,
//         sfs: -500,
//     };

//     let analyzer = Analyzer::new(data, weights);

//     let layout =
//         Layout::load("./layouts/heatmap1.dof").expect("this layout is valid and exists, soooo");

//     let mut depth2 = time_this::time!((0..9600)
//         .into_par_iter()
//         .map(|_| {
//             let starting_layout = layout.random();

//             let (layout, _) = analyzer.greedy_improve(starting_layout);
//             analyzer.greedy_depth2_improve(layout).1
//         })
//         .collect::<Vec<_>>());

//     depth2.sort_unstable_by(|s1, s2| s2.cmp(s1));

//     // let mut annealing = time_this::time!((0..(41000 * 17 * 17 / 30))
//     //     .into_par_iter()
//     //     .map(|_| {
//     //         let starting_layout = layout.random();

//     //         analyzer.annealing_improve(starting_layout, 20_500_000_000_000.0, 0.983, 4000).1
//     //     })
//     //     .collect::<Vec<_>>());

//     // annealing.sort_unstable_by(|s1, s2| s2.cmp(s1));

//     // let mut greedy = time_this::time!((0..410_000)
//     //         .into_par_iter()
//     //         .map(|_| {
//     //             let starting_layout = layout.random();

//     //             analyzer.greedy_improve(starting_layout).1
//     //         })
//     //         .collect::<Vec<_>>());

//     // greedy.sort_unstable_by(|s1, s2| s2.cmp(s1));

//     // let mut alternative = time_this::time!((0..(400_000 * 280 / 150 * 17 / 18))
//     //     .into_par_iter()
//     //     .map(|_| {
//     //         let starting_layout = layout.random();

//     //         analyzer.always_better_swap(starting_layout).1
//     //     })
//     //     .collect::<Vec<_>>());

//     // alternative.sort_unstable_by(|s1, s2| s2.cmp(s1));

//     let mut res = String::from("depth2\n");

//     // greedy.into_iter()
//     //     .zip(alternative)
//     //     .zip(annealing)
//     //     .for_each(|((g, al), an)| {
//     //         let _ = writeln!(&mut res, "{an},{g},{al}");
//     //     });

//     depth2.into_iter().for_each(|d2| {
//         let _ = writeln!(&mut res, "{d2}");
//     });

//     let mut f = std::fs::OpenOptions::new()
//         .write(true)
//         .truncate(true)
//         .create(true)
//         .open("./messing-with-data/greedy_data3.csv")
//         .unwrap();

//     f.write_all(res.as_bytes()).unwrap();
// }
