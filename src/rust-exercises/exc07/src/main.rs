use std::slice;

fn main() {
    let mut numbers = vec![1, 4, 8, 1, -7, 2, 4, 6, 7, 9, 14, 28, 1, 7, 9];

    // numbers dizisi 3. indexten ikiye bölünüyor
    let (left_slice, right_slice) = split_array_from(&mut numbers, 3);

    println!("Left slice values: {:?}", left_slice);
    println!("Right slice values: {:?}", right_slice);

    // left_slice dilimindeki ilk elemanı değiştiriyoruz
    // bu değişiklik orijinal numbers dizisini de etkileyecektir
    left_slice[0] = 345;
    println!("After changed the left slice: {:?}", numbers);
}

/// Bu fonksiyon, verilen `values` dilimini `index` konumunda ikiye böler
/// ve iki ayrı dilim olarak döner.
///
/// # Safety
///
/// Bu fonksiyon unsafe kod kullanır, bu nedenle dikkatli olunmalıdır.
///
/// # Arguments
/// - `values`: Bölünecek olan tamsayı dilimi.
/// - `index`: Bölme işleminin gerçekleşeceği konum.
///
/// # Returns
/// İki ayrı tamsayı dilimi olarak döner.
fn split_array_from(values: &mut [i32], index: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    // ptr değişkeni, values diliminin başlangıç adresini tutan bir işaretçidir(pointer).
    let ptr = values.as_mut_ptr();

    /*
        from_raw_parts_mut fonksiyonu unsafe türdendir ve bu nedenle
        unsafe kod bloğu içerisinde çalıştırılması gerekir.
    */
    unsafe {
        // ptr ile tutulan adresten başlayarak index uzunluğunda bir dilim oluşturur.
        let left = slice::from_raw_parts_mut(ptr, index);
        // index noktasından başlayarak len - index uzunluğunda bir dilim oluşturur.
        let right = slice::from_raw_parts_mut(ptr.add(index), len - index);
        (left, right)
    }
}
