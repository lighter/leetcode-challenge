# The is_bad_version API is already defined for you.
# @param {Integer} version
# @return {boolean} whether the version is bad
# def is_bad_version(version):

# @param {Integer} n
# @return {Integer}
def first_bad_version(n)
  binary_search(1, n + 1)
end

def binary_search(first, last)
  mid = (first + last) / 2

  if is_bad_version(mid) && !is_bad_version(mid - 1)
    mid
  elsif !is_bad_version(mid)
    binary_search(mid + 1, last)
  else
    binary_search(first, mid)
  end
end