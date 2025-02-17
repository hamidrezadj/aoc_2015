type Weight = u64;

fn main() {
    let weights: Vec<Weight> = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| {
            line.parse()
                .expect("Package weight could not fit in an unsigned 64 bit integer")
        })
        .collect();
    let weights_sum: Weight = weights.iter().sum();
    assert_eq!(weights_sum % 4, 0, "Sum of weights not divisible by three");
    let section_weight = weights_sum / 4;
    let combinations = combinations(&weights, section_weight, &[]);
    let center_package_count = combinations
        .iter()
        .min_by_key(|combination| combination.len())
        .expect("Could not find valid center compartment package combination")
        .len();
    let output = combinations
        .into_iter()
        .filter(|combination| combination.len() == center_package_count)
        .map(|combination| {
            combination
                .iter()
                .map(|weight_index| weights[*weight_index])
                .product::<Weight>()
        })
        .min()
        .unwrap();
    println!("{}", output);
}

fn combinations(
    weights: &[Weight],
    target_section_weight: Weight,
    weights_indicies_under_inspection: &[usize],
) -> Vec<Vec<usize>> {
    let under_inspection_weight = weights_indicies_under_inspection
        .iter()
        .map(|idx| weights[*idx])
        .sum::<Weight>();
    if under_inspection_weight == target_section_weight {
        return vec![weights_indicies_under_inspection.to_vec()];
    }
    let last_index = weights_indicies_under_inspection.last().copied();
    let starting_index = match last_index {
        Some(i) => i + 1,
        None => 0,
    };
    (starting_index..weights.len())
        .filter(|index| under_inspection_weight + weights[*index] <= target_section_weight)
        .flat_map(|index| {
            let mut new_under_inspection = weights_indicies_under_inspection.to_vec();
            new_under_inspection.push(index);
            combinations(weights, target_section_weight, &new_under_inspection)
        })
        .collect()
}
