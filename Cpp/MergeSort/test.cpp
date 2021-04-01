#include "merge-sort.hpp"

#include <algorithm>
#include <random>
#include <functional>
#include <cassert>




void test()
{
	const std::default_random_engine rng;
	const std::uniform_int_distribution dist{ 0 };
	std::vector<int> data(1'024);
	std::generate(data.begin(), data.end(), std::bind(dist, rng));

    auto toTest = data;
    mergeSort(std::span<int>{ toTest });

    auto tested = data;
    std::sort(tested.begin(), tested.end());

    assert(toTest == tested);
}