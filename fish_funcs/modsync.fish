function modsync
    set p $argv[1]
    echo "" > src/$p.rs
    for l in (ls src/$p | grep .rs)
        set n (echo $l | string sub -e -3)
        echo "pub mod $n;" >> src/$p.rs
    end
end
