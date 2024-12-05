for i in $(seq 1 15); do
    ./run.sh "$i"
    mkdir -p "analysis/480p$i"
    mv outputs/* "analysis/480p$i/"
done

