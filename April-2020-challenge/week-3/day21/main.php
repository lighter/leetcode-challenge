<?

/**
 * // This is the BinaryMatrix's API interface.
 * // You should not implement it, or speculate about its implementation
 * class BinaryMatrix {
 *     public function get($x, $y) {} @return Integer
 *     public function dimensions() {} @return Integer[]
 * }
 */

class Solution
{
  public function findFirstOne($binaryMatrix, $row)
  {
    $result = -1;
    $start = 0;
    $end = $binaryMatrix->dimensions()[1];

    while (true) {
      if ($start == $end) {
        return -1;
      }

      $mid = floor(($start + $end) / 2);

      if ($binaryMatrix->get($row, $mid) == 1) {
        if ($mid == 0) {
          return 0;
        }

        if ($binaryMatrix->get($row, $mid - 1) == 0) {
          return $mid;
        } else {
          $end = $mid;
        }
      } else {
        $start = $mid + 1;
      }
    }
  }
  /**
   * @param BinaryMatrix $binaryMatrix
   * @return Integer
   */
  public function leftMostColumnWithOne($binaryMatrix)
  {
    $ans = -1;
    [$i, $j] = $binaryMatrix->dimensions();

    for ($row = 0; $row < $i; $row++) {
      $leftOne = $this->findFirstOne($binaryMatrix, $row);

      if ($leftOne != -1) {
        if ($ans == -1 || $leftOne < $ans) {
          $ans = $leftOne;
        }
      }
    }

    return $ans;
  }
}