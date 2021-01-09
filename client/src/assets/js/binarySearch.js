function binarySearchInsertImpl(array, compare, begin, end) {
  if (begin === end) return begin;
  const middle = begin + ((end - begin) >>> 1);
  const order = compare(array[middle]);
  if (order < 0) {
    return binarySearchInsertImpl(array, compare, begin, middle);
  } else if (order > 0) {
    return binarySearchInsertImpl(array, compare, middle + 1, end);
  } else {
    return middle;
  }
}

function binarySearchFindImpl(array, compare, begin, end) {
  if (begin === end) return null;
  const middle = begin + ((end - begin) >>> 1);
  const order = compare(array[middle]);
  if (order < 0) {
    return binarySearchFindImpl(array, compare, begin, middle);
  } else if (order > 0) {
    return binarySearchFindImpl(array, compare, middle + 1, end);
  } else {
    return middle;
  }
}

export function binarySearchInsert(array, compare) {
  return binarySearchInsertImpl(array, compare, 0, array.length);
}

export function binarySearchFind(array, compare) {
  return binarySearchFindImpl(array, compare, 0, array.length);
}
