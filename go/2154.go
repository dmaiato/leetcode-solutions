func findFinalValue(nums []int, original int) int {
    
    values := make(map[int]bool)
    for _, num := range nums {
        values[num] = true
    }

    copy := original
    for {
        if _, ok := values[copy]; ok {
            copy *= 2
            continue
        }
        break
    }
    return copy
}
