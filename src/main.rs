use rand::prelude::*;

const MIN_REPS: u8 = 5;
const MAX_REPS: u8 = 15;

fn build_exercise_list() -> Vec<String> {
    let mut exercises = Vec::new();
    exercises.push("Crunches".to_owned());
    exercises.push("Jumping Jacks".to_owned());
    exercises.push("Lunges".to_owned());
    exercises.push("Push Ups".to_owned());
    exercises.push("Squats".to_owned());
    return exercises;
}

fn main() {
    let mut rng = rand::thread_rng();
    let exercises = build_exercise_list();

    let reps = rng.gen_range(MIN_REPS, MAX_REPS + 1);
    let exercise_choice = rng.gen_range(0, exercises.len());
    let exercise = &exercises[exercise_choice];

    println!("Up for another round, I see!");
    println!("Let's do {} reps of {}", reps, exercise);
}
