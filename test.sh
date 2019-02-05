#!/bin/zsh
if [ $# -ne 1 ] || [[ ! $1 =~ ^(3|4)$ ]] ; then
	echo "usage:\t$0 <3|4>\n
\t$0 3 (test puzzles 3x3)
\t$0 4 (test puzzles 4x4)"
	exit 1
fi

ls test/solvable3* test/solvable4* 1> /dev/null 2>&1
if [ $? -ne 0 ] ; then
	echo "need puzzle with name test/solvable3* test/solvable4*"
	echo 'tips:\n\tfor e in {00..10} ; do 
		python npuzzle-gen.py 3 -s > test/solvable3_$e;
		python npuzzle-gen.py 4 -s > test/solvable4_$e;
	done'
	exit 1
fi

cargo build --release

commit_id="`git rev-parse master | head -c 8`"
time="`date +"%s"`"
file_name="time_test_$1_${time}:${commit_id}.log"

echo "#########################################################################" > ${file_name}
echo "# commit: ${commit_id}" >> ${file_name}
echo "#" >> ${file_name}
echo "# for e in \`ls test/solvable$1\*\` ; do" >> ${file_name}
echo "#	echo \"\\\n\$e:\" ;" >> ${file_name}
echo '#	{ time ./target/release/npuzzle $e } 2>&1 | grep -i "len\|cpu\|number\|state";' >> ${file_name}
echo '# done  > ${file_name}' >> ${file_name}
echo "#" >> ${file_name}
echo "#" >> ${file_name}
echo "# # cat $file_name | grep target | awk -F ' ' '{print $3}'" >> ${file_name}
echo "#########################################################################\n\n" >> ${file_name}

for e in `ls test/solvable$1*` ; do
	echo "\n$e:" ;
	{ time ./target/release/npuzzle $e } 2>&1 | grep -i 'len\|cpu\|number\|state';
 done >> ${file_name}

echo "${file_name} generate"