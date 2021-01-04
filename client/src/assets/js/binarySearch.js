function binarySearchImpl(array, compare, begin, end) {
  if (begin === end) return begin;
  const middle = begin + ((end - begin) >>> 1);
  const order = compare(array[middle]);
  if (order < 0) {
    return binarySearchImpl(array, compare, begin, middle);
  } else if (order > 0) {
    return binarySearchImpl(array, compare, middle + 1, end);
  } else {
    return middle;
  }
}

export default function(array, compare) {
  return binarySearchImpl(array, compare, 0, array.length);
}
