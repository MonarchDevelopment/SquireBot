#! /usr/bin/bash
# Check syntax
cd ../
find . | grep .py$ | xargs --verbose -n 1 python3 -m py_compile
code=$?
echo $code
if (( $code >= 1 ));
then
    echo "Invalid Syntax (see output above)"
    exit 1
fi

#find . | grep .py$ | xargs --verbose -n 1 python3 -m pylint

cd tests

# Run tests
python3 testRunner.py
code=$?
echo $code
if (( $code >= 1 ));
then
    echo "Some tests failed (see output above)"
    exit 2
fi
