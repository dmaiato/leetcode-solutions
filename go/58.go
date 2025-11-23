func lengthOfLastWord(s string) int {
    
    counter := 0
    size := len(s)

    for i := size -1; i >= 0; i-- {
        if s[i] == ' ' {
            if counter > 0 {
                return counter
            }
            continue
        }
        counter++
    }
    return counter
}
