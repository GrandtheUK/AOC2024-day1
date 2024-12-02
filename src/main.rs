use std::{fs::File, io::Read, iter::zip};

fn main() {
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];
    let mut file: File = File::open("input.txt").unwrap();
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).unwrap();
    for item in contents.split("\n") {
        let mut i: i32 = 0;
        for part in item.split("   ") {
            // println!("part: {}",part);
            if let Ok(elem) = part.parse::<i32>() {
                if i == 0 {
                    list1.insert(list1.len(), elem);
                    i+=1;
                } else {
                    list2.insert(list2.len(), elem);
                    i-=1;
                }
            }
        }
    }
    // Sort the lists to get them in size order
    list1.sort();
    list2.sort();

    let mut dist: Vec<i32> = vec![];
    // Zip the 2 lists and calculate each distance
    for pair in zip(list1.clone(), list2.clone()) {
        let res: i32 = pair.0 - pair.1;
        dist.insert(dist.len(), res.abs());
    }

    let mut total_distance: i32 = 0;
    // for element in the distance list add to final result
    for elem in dist {
        total_distance+=elem;
    }
    println!("Total Distance is {}", total_distance);

    // Part Two
    let mut similarity_score: i32 = 0;

    for elem in list1.clone() {
        let count =list2.iter() // iterate the list so we can filter
            .filter(|x| **x == elem) // filter list1 for values matching the element from list2
            .count(); // count up the values left
        similarity_score+=elem*(count as i32); // treat count usize as an integer and work out the similarity score for the element add add it to the total
    }
    println!("Similarity_score is {}",similarity_score);
}
