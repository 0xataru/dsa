# Median of Two Sorted Arrays - Optimal Go Solution

## Problem Summary
Given two sorted arrays `nums1` and `nums2`, find the median of the combined sorted arrays in **O(log(min(m,n)))** time complexity.

## Solution Approach: Binary Search Partitioning

### Algorithm Overview
The optimal solution uses binary search partitioning to achieve O(log(min(m,n))) time complexity. Instead of merging the arrays (which would be O(m+n)), we partition both arrays such that all elements on the left are smaller than or equal to all elements on the right, and the left partition contains exactly half of all elements.

### Key Insight
The median divides a sorted sequence into two equal parts. For two arrays, we need to find a partition where:
1. Left partition has exactly `(m + n + 1) / 2` elements
2. `max(left_partition) ≤ min(right_partition)`

### Core Algorithm Logic

We perform binary search on the smaller array to find the correct partition point. For each partition attempt:
- Calculate how many elements from each array should be in the left partition
- Check if the partition is valid using the cross-comparison rule
- Adjust the binary search bounds based on the validation result

### Step-by-Step Algorithm

1. **Ensure Optimal Search**: Always search on the smaller array for O(log(min(m,n))) complexity
2. **Binary Search Setup**: Initialize left=0, right=m, and calculate target left partition size
3. **Partition Calculation**: For each mid-point, determine partition positions in both arrays
4. **Validation**: Check if `maxLeft1 ≤ minRight2` and `maxLeft2 ≤ minRight1`
5. **Result**: If valid, calculate median; otherwise, adjust search bounds

### Implementation Details

```go
func findMedianSortedArrays(nums1, nums2 []int) float64 {
    // Ensure nums1 is smaller for optimal performance
    if len(nums1) > len(nums2) {
        nums1, nums2 = nums2, nums1
    }

    m, n := len(nums1), len(nums2)
    total := m + n
    half := (total + 1) / 2

    left, right := 0, m

    for left <= right {
        partition1 := (left + right) / 2
        partition2 := half - partition1

        // Handle boundary conditions with sentinel values
        maxLeft1 := math.MinInt32
        if partition1 > 0 {
            maxLeft1 = nums1[partition1-1]
        }

        minRight1 := math.MaxInt32
        if partition1 < m {
            minRight1 = nums1[partition1]
        }

        maxLeft2 := math.MinInt32
        if partition2 > 0 {
            maxLeft2 = nums2[partition2-1]
        }

        minRight2 := math.MaxInt32
        if partition2 < n {
            minRight2 = nums2[partition2]
        }

        // Check partition validity
        if maxLeft1 <= minRight2 && maxLeft2 <= minRight1 {
            // Valid partition found
            if total%2 == 1 {
                return float64(max(maxLeft1, maxLeft2))
            } else {
                leftMax := max(maxLeft1, maxLeft2)
                rightMin := min(minRight1, minRight2)
                return float64(leftMax+rightMin) / 2.0
            }
        } else if maxLeft1 > minRight2 {
            right = partition1 - 1 // Move left
        } else {
            left = partition1 + 1 // Move right
        }
    }

    return 0.0 // Should never reach here
}
```

### Why This Approach Works

1. **Partition Property**: By maintaining the invariant that left partition has exactly `(m+n+1)/2` elements, we ensure the median lies at the partition boundary
2. **Cross Validation**: The condition `maxLeft1 ≤ minRight2 && maxLeft2 ≤ minRight1` ensures all left elements ≤ all right elements
3. **Binary Search Efficiency**: Searching on the smaller array guarantees logarithmic time complexity

### Complexity Analysis

#### Time Complexity: O(log(min(m,n)))
- We perform binary search on the smaller array
- Each iteration does O(1) work
- Maximum iterations: log(min(m,n))

#### Space Complexity: O(1)
- Only uses constant extra space for variables
- No additional data structures required
- Input arrays are not modified

### Comparison with Alternative Approaches

| Approach | Time Complexity | Space Complexity | Description |
|----------|----------------|------------------|-------------|
| **Binary Search Partition** | O(log(min(m,n))) | O(1) | ✅ **Optimal** |
| Merge and Find | O(m+n) | O(m+n) | Merge arrays, find middle |
| Two Pointers | O(m+n) | O(1) | Traverse to middle position |
| Binary Search on Answer | O((m+n)log(range)) | O(1) | Search for median value |

### Edge Cases Handled

1. **Empty Arrays**: When one array is empty, median comes from the other array
2. **Single Elements**: Arrays with just one element each
3. **Unequal Sizes**: Efficiently handles arrays of very different sizes
4. **Duplicate Values**: Correctly processes arrays with repeated elements
5. **Negative Numbers**: Works with negative integers and mixed signs
6. **Extreme Values**: Uses `math.MinInt32/MaxInt32` as sentinel values

### Go-Specific Implementation Details

#### Efficient Array Swapping
```go
// Zero-cost array reference swapping
if len(nums1) > len(nums2) {
    nums1, nums2 = nums2, nums1
}
```

#### Sentinel Value Usage
```go
// Safe boundary handling with math package constants
maxLeft1 := math.MinInt32
if partition1 > 0 {
    maxLeft1 = nums1[partition1-1]
}
```

#### Helper Functions
```go
func min(a, b int) int {
    if a < b { return a }
    return b
}

func max(a, b int) int {
    if a > b { return a }
    return b
}
```

### Performance Characteristics

- **Best Case**: O(1) when one array is much smaller than the other
- **Average Case**: O(log(min(m,n))) for typical inputs
- **Worst Case**: O(log(min(m,n))) regardless of data distribution
- **Memory Usage**: Constant extra space, memory-efficient

### Testing Strategy

The implementation includes comprehensive test coverage:

```go
testCases := []struct {
    nums1    []int
    nums2    []int
    expected float64
    name     string
}{
    {[]int{1, 3}, []int{2}, 2.0, "Example 1"},
    {[]int{1, 2}, []int{3, 4}, 2.5, "Example 2"},
    {[]int{}, []int{1}, 1.0, "Empty first array"},
    {[]int{2}, []int{}, 2.0, "Empty second array"},
    // ... more test cases
}
```

### Mathematical Correctness

The algorithm is mathematically sound because:

1. **Median Definition**: For combined sorted array of length n:
   - If n is odd: median = middle element
   - If n is even: median = average of two middle elements

2. **Partition Invariant**: Left partition always contains exactly ⌊(m+n+1)/2⌋ elements

3. **Ordering Property**: Valid partition ensures sorted order across partition boundary

### Advanced Optimization Notes

1. **Branch Prediction**: Simple conditional logic optimizes well in Go
2. **Memory Access**: Sequential array access patterns are cache-friendly  
3. **Integer Arithmetic**: All operations use efficient integer math
4. **Slice Efficiency**: Go slices provide zero-cost length operations

### Error Handling and Robustness

The solution handles all constraint-specified scenarios:
- Array lengths from 0 to 1000
- Element values from -10⁶ to 10⁶  
- Combined array length from 1 to 2000
- Guaranteed solution existence per problem constraints

### Performance Benchmarking

The implementation includes performance testing:
- Large input arrays (500 + 500 elements)
- Asymmetric cases (1 + 999 elements)
- Execution time measurement
- Complexity verification

### Alternative Go Implementations Considered

1. **Using sort.SearchInts**: Could binary search for median value, but less efficient
2. **Channel-based merging**: Elegant but O(m+n) complexity
3. **Recursive approach**: Clean but potentially stack-intensive
4. **Interface-based**: More generic but adds overhead

### go.mod Configuration
```go
module median-of-two-sorted-arrays

go 1.21
```

### Conclusion

This Go implementation achieves the optimal solution for the Median of Two Sorted Arrays problem:

- **Time Optimal**: O(log(min(m,n))) complexity meets the problem requirement
- **Space Optimal**: O(1) extra space usage
- **Robust**: Handles all edge cases and constraints correctly
- **Efficient**: Leverages Go's strengths in simplicity and performance
- **Readable**: Clear logic flow with comprehensive documentation
- **Testable**: Extensive test coverage validates correctness

The solution demonstrates how Go's simplicity and efficiency can be leveraged to implement complex algorithms cleanly while maintaining optimal performance characteristics. The binary search partitioning approach represents the state-of-the-art solution for this fundamental computer science problem. 