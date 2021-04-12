fun checkMagazine(magazine: Array<String>, note: Array<String>): String {
    val magazineWords = magazine.groupingBy { it }.eachCount()
    val noteWords = note.groupingBy { it }.eachCount()

    for ((word, freq) in noteWords) {
        if (magazineWords.getOrDefault(word, 0) < freq) {
            return "No"
        }
    }

    return "Yes"
}

println(checkMagazine(arrayOf("attack", "at", "dawn"), arrayOf("attack", "at", "dawn")) == "Yes")
println(checkMagazine(arrayOf("attack", "at", "dawn"), arrayOf("Attack", "at", "dawn")) == "No")
println(checkMagazine(arrayOf("give", "me", "one", "grand", "today", "night"), arrayOf("give", "one", "grand", "today")) == "Yes")
println(checkMagazine(arrayOf("two", "times", "three", "is", "not", "four"), arrayOf("two", "times", "two", "is", "four")) == "No")
println(checkMagazine(arrayOf("ive", "got", "a", "lovely", "bunch", "of", "coconuts"), arrayOf("ive", "got", "some", "coconuts")) == "No")
println(checkMagazine(arrayOf("apgo", "clm", "w", "lxkvg", "mwz", "elo", "bg", "elo", "lxkvg", "elo", "apgo", "apgo", "w", "elo", "bg"),
        arrayOf("elo", "lxkvg", "bg", "mwz", "clm", "w")) == "Yes")