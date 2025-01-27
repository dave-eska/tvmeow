use bevy::prelude::*;

fn main(){
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, print_names)
        .run();
}

pub fn setup(mut cmd: Commands){
    cmd.spawn(Person{
        name: "daveeska".to_string()
    });
    cmd.spawn(Person{
        name: "yesy".to_string()
    });
}

pub fn print_names(person_q: Query<&Person>){
    for person in person_q.iter(){
        println!("{}", person.name);
    }
}

#[derive(Component)]
pub struct Person{
    pub name: String
}
