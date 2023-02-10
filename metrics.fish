#!/opt/homebrew/bin/fish
rm Metrics.csv
cargo build --release --quiet
clear
set iter_count 0
set eval_count 0

set algos b l a1
set algo_count (count $algos)

set start_eval (seq 0 2 14)
set med_eval (seq 0 20 110)
set large_eval (seq 0 700 4000)

set weights (seq 1 0.2 6)

set eval_count (math $eval_count + (count $start_eval))
set eval_count (math $eval_count + (count $med_eval))
set eval_count (math $eval_count + (count $large_eval))
set eval_count (math $eval_count x (math $algo_count + (count $weights)))
for end in $start_eval
	for a in $algos
		clear
		set iter_count (math $iter_count + 1)
		echo (math round (math $iter_count / $eval_count x 100))% \t 01.txt \t $end : $a \t $iter_count / $eval_count
		target/release/pathfinder maps/01.txt 0 0 $end $end -$a --quiet --metrics
	end
	for w in $weights
		clear
		set iter_count (math $iter_count + 1)
		echo (math round (math $iter_count / $eval_count x 100))% \t 01.txt \t $end : a2 \($w\) \t $iter_count / $eval_count
		target/release/pathfinder maps/01.txt 0 0 $end $end -a2 --quiet --metrics $w
	end
end
for end in $med_eval
	for a in $algos
		clear
		set iter_count (math $iter_count + 1)
		echo (math round (math $iter_count / $eval_count x 100))% \t 02.txt \t $end : $a \t $iter_count / $eval_count
		target/release/pathfinder maps/02.txt 0 0 $end $end -$a --quiet --metrics
	end
	for w in $weights
		clear
		set iter_count (math $iter_count + 1)
		echo (math round (math $iter_count / $eval_count x 100))% \t 01.txt \t $end : a2 \($w\) \t $iter_count / $eval_count
		target/release/pathfinder maps/02.txt 0 0 $end $end -a2 --quiet --metrics $w
	end
end
for end in $large_eval
	for a in $algos
		clear
		set iter_count (math $iter_count + 1)
		echo (math round (math $iter_count / $eval_count x 100))% \t 03.txt \t $end : $a \t $iter_count / $eval_count
		target/release/pathfinder maps/03.txt 0 0 $end $end -$a --quiet --metrics
	end
	for w in $weights
		clear
		set iter_count (math $iter_count + 1)
		echo (math round (math $iter_count / $eval_count x 100))% \t 01.txt \t $end : a2 \($w\) \t $iter_count / $eval_count
		target/release/pathfinder maps/03.txt 0 0 $end $end -a2 --quiet --metrics $w
	end
end
