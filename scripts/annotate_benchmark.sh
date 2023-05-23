pid=$1
if [ -z "$pid" ]
then
    echo "please provide <pid>"
    return;
fi

filename="callgrind.out.$pid"

if [ ! -f $filename ]
then
    echo "file does not exist"
    return;
fi

callgrind_annotate --tree=both $filename > "${filename}_annotated"