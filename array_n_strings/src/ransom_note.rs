pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    if ransom_note.len() > magazine.len() {
        return false;
    }
    let mut note_arr = [0i32; 26];
    let mut mag_arr = [0i32; 26];

    let ransom_note = ransom_note.as_bytes();
    let magazine = magazine.as_bytes();

    for i in 0..magazine.len() {
        if i < ransom_note.len() {
            note_arr[(ransom_note[i] - b'a') as usize] += 1;
        }
        mag_arr[(magazine[i] - b'a') as usize] += 1;
    }
    for val in ransom_note {
        let letter = (val - b'a');
        let mag = mag_arr[letter as usize];
        let note = note_arr[letter as usize];
        if mag < note {
            return false;
        }
    }
    true
}
