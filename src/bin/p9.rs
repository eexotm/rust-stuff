fn main() {
    iterator_show();
    iterator_consumers();
}

fn iterator_show() {
    let v1 = vec![1,2,3,4];
    let mut v1_iter = v1.iter();

    
    loop {
        let value = v1_iter.next();
        if let Some(val) = value {
            println!("value; {:?}", val);
        }else {
            println!("hit none");
            break;
        }
        
    }
}

fn iterator_consumers() {
    let v1 = vec![1,2,3,4,5];
    let v2:Vec<i32> = v1.iter().map(|x| x + 4).collect();
    for i in v2.iter() {
        println!("v2 value: {i}")
    }
}
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}
fn select_by_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
fn select_by_brand(shoes: Vec<Shoe>, shoe_brand: String) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.style == shoe_brand).collect()
}

#[cfg(test)]
mod tests{ 
    use super::*;
    #[test]
    fn filter_by_size() {
        let shoe_vec = vec![Shoe {
            size: 9,
            style: String::from("Adidas")
        },
        Shoe {
            size: 8,
            style: String::from("Nike")
        },
        Shoe {
            size: 11,
            style: String::from("Adidas")
        },
        Shoe {
            size: 9,
            style: String::from("Polo")
        },
        Shoe {
            size: 11,
            style: String::from("Mountain")
        },
        Shoe {
            size: 10,
            style: String::from("Polo")
        },
            
        ];
        let my_size = select_by_size(shoe_vec, 10);
        assert_eq!(my_size, vec![Shoe {
            size: 10,
            style: String::from("Polo")
        },])
    }
    #[test]
    fn filter_by_size2() {
        let shoe_vec = vec![
        Shoe {
            size: 9,
            style: String::from("Adidas")
        },
        Shoe {
            size: 8,
            style: String::from("Nike")
        },
        Shoe {
            size: 11,
            style: String::from("Adidas")
        },
        Shoe {
            size: 9,
            style: String::from("Polo")
        },
        Shoe {
            size: 11,
            style: String::from("Mountain")
        },
        Shoe {
            size: 10,
            style: String::from("Polo")
        }];
        let my_size = select_by_size(shoe_vec, 11);
        
        assert_eq!(my_size, vec![Shoe {
            size: 11,
            style: String::from("Adidas")
        },Shoe {
            size: 11,
            style: String::from("Mountain")
        },])
    }
    #[test]
    fn filter_by_brand() {
        let shoe_vec = vec![
        Shoe {
            size: 9,
            style: String::from("Adidas")
        },
        Shoe {
            size: 8,
            style: String::from("Nike")
        },
        Shoe {
            size: 11,
            style: String::from("Adidas")
        },
        Shoe {
            size: 9,
            style: String::from("Polo")
        },
        Shoe {
            size: 11,
            style: String::from("Mountain")
        },
        Shoe {
            size: 10,
            style: String::from("Polo")
        }];
        let my_choice = select_by_brand(shoe_vec, String::from("Adidas"));
        assert_eq!(my_choice, vec![Shoe {
            size: 9,
            style: String::from("Adidas")
        },Shoe {
            size: 11,
            style: String::from("Adidas")
        },])
    }

}

/* MENTAL MODEL
ITER GOES THRU THE ELEMENTS, WHEN WE ADD MODIFIERS LIKE .MAP AND .FILTER
OUR .ITER GIVES EACH ELEMENT TO THE FILTER OR THE MAP TO THEN 
.COLLECT TO COLLECT IT INTO A NEWER VECTOR.

*/ 