#!/bin/zsh
set -e
# set -x

cpp_imagezero_bin=../../imagezero-sys/imagezero-cpp/build/iz
rust_cpp_imagezero_bin=../../imagezero-sys/target/release/imagezero

for pic in $(ls *.ppm); do
  base=$pic:t:r
  echo " ---- ${base} ---- "
  time ${cpp_imagezero_bin} c ${pic} ${base}.cpp.iz
  time ${rust_cpp_imagezero_bin} -c ${pic} ${base}.rust_cpp.iz
  diff ${base}.cpp.iz ${base}.rust_cpp.iz

  time ${cpp_imagezero_bin} d ${base}.cpp.iz ${base}.cpp.new
  time ${rust_cpp_imagezero_bin} -d ${base}.rust_cpp.iz ${base}.rust_cpp.new
  diff ${pic} ${base}.cpp.new
  diff ${pic} ${base}.rust_cpp.new
done

rm *.iz
rm *.new
