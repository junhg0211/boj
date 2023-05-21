rustc -o a.exe $args[0]

if ($?) {
    .\a.exe
    rm a.exe
    rm a.pdb
}
