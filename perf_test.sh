#!/bin/bash
set -e

declare -a TESTS=("benchmark_render_scene" "benchmark_parse_scene" "benchmark_save_scene" "benchmark_triangles_render_1")
TIMESTAMP=$(date +"%Y-%m-%d %H:%M")
GIT_HASH=$(git rev-parse --short HEAD)
mkdir -p perf

CSV_FILE="perf/perf_results.csv"
HEADER="timestamp,commit"

# Construire l'en-tête si le fichier n'existe pas
if [ ! -f "$CSV_FILE" ]; then
    for TEST_NAME in "${TESTS[@]}"; do
        HEADER+=",${TEST_NAME}_ms"
    done
    echo "$HEADER" > "$CSV_FILE"
fi

# Initialiser ligne de données
LINE="$TIMESTAMP,$GIT_HASH"

# Lancer chaque test
for TEST_NAME in "${TESTS[@]}"; do
    echo "Running test: $TEST_NAME"

    # Exécuter le test et capturer la sortie
    cargo test --release --test render_bench -- --nocapture --exact "$TEST_NAME" > "perf/raw_output_${TEST_NAME}.txt"

    # Extraire le temps
    RENDER_TIME=$(grep "RenderTimeMs:" "perf/raw_output_${TEST_NAME}.txt" | awk '{print $2}')
    LINE+=",$RENDER_TIME"

    # Flamegraph
    echo "Generating flamegraph for $TEST_NAME"
    sudo cargo flamegraph --test render_bench --release --root -- --exact "$TEST_NAME"
    mv flamegraph.svg "perf/flamegraph-${GIT_HASH}-${TEST_NAME}.svg"
done

# Ajouter la ligne au CSV
echo "$LINE" >> "$CSV_FILE"

echo "Performance results recorded in $CSV_FILE"
