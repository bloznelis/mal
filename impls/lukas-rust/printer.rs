use crate::types::MalType;

pub fn pr_str(mal: &MalType) -> String {
    match mal {
        MalType::MalAtom(value) => value.to_owned(),
        MalType::MalList(list) => {
            let inner: String = list
                .iter()
                .map(|x| pr_str(x))
                .collect::<Vec<String>>()
                .join(" ");

            format!("({})", inner)
        }
    }
}
