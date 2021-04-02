// Used in join_halves.
use std::iter::once;

fn main() {
    // We first create an iterator of random floats.
    let unsorted = (1..1000).map(|_| rand::random::<f32>()).collect();

    // We sort the vector using merge sort.
    let _sorted = merge_sort(unsorted);
}

fn merge_sort(vector: Vec<f32>) -> Vec<f32> {
    if vector.len() == 1 {
        // Base case: If our argument is of length 1, we simply return it.
        vector
    } else {
        // Our vector has more than two elements. We start by splitting it.
        let (left, right) = vector.split_at(vector.len() / 2);
        // We recursively sort the two halves of the vectors, then return
        // the two joined vectors.
        join_halves(merge_sort(Vec::from(left)), merge_sort(Vec::from(right)))
    }
}

fn join_halves(left: Vec<f32>, right: Vec<f32>) -> Vec<f32> {
    // The index in the right vector, to avoid inserting twice the same item.
    let mut right_index = 0;

    left.into_iter().flat_map (
        // We process the elements from the left vector one by one.
        |left_element| {
            // We first have to find all the elements from the right vector that
            // should come before the left_element.
            let to_insert: Vec<f32> = right
                // We skip the elements that have already been inserted once.
                .iter().skip(right_index)
                .take_while(|&right_element| *right_element < left_element)
                .map(
                    // We dereference all the elements.
                    |element| *element,
                )
                // We need to collect the result into a Vec for the len method,
                // as count would consume the iterator.
                .collect();

            // We increment the index of the right vector.
            right_index += to_insert.len();

            // We return the sorted iterator that should be sent to the
            // merged Vector.
            to_insert.into_iter().chain(once(left_element))
        }
    )
    // We have to consume all the references to the right vector from the
    // flat_map before we can call into_iter and mutably borrow it. Here we
    // collect the iterator as a Vec<f32>, which we then recast into a new
    // iterator. The point of this manipulation is to force the evaluation
    // of the FlatMap before the call to right.into_iter.
    .collect::<Vec<f32>>().into_iter()
    // We can directly chain in the remaining elements from the right vector
    // that are larger than any of the elements found in the left vector.
    .chain(right.into_iter().skip(right_index))
    // We collect the final result to return it.
    .collect()
}
