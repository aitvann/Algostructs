#include <iostream>
#include <span>
#include <cassert>
#include <vector>




template<typename T>
void merge(std::span<T> srcLeft, std::span<T> srcRight, std::span<T> dst)
{
	assert(srcLeft.size() + srcRight.size() == dst.size());

	std::size_t i{ 0 };
	std::size_t j1{ 0 };
	std::size_t j2{ 0 };

	while (j1 < srcLeft.size() && j2 < srcRight.size())
		if (srcLeft[j1] < srcRight[j2])
			dst[i++] = srcLeft[j1++];
		else
			dst[i++] = srcRight[j2++];

	while (j1 < srcLeft.size())
		dst[i++] = srcLeft[j1++];

	while (j2 < srcRight.size())
		dst[i++] = srcRight[j2++];
}

template <typename T>
void mergeByStep(std::span<T> src, std::span<T> dst, std::size_t step)
{
	assert(src.size() == dst.size());

	for (std::size_t offset{ 0 }; offset < src.size(); offset += step)
	{
		const std::size_t count{ std::min(step, src.size() - offset) };
		const std::size_t mid{ std::min(count, step / 2) };
		const auto subsrc = src.subspan(offset, count);
		const auto subdst = dst.subspan(offset, count);
		merge(subsrc.subspan(0, mid), subsrc.subspan(mid), subdst);
	}
}

template <typename T>
void mergeSort(std::span<T> data, std::span<T> buf)
{
	assert(data.size() == buf.size());

	if (data.size() < 2)
		return;

	bool swap{ false };
	std::size_t subSpanSize{ 2 };
	while (subSpanSize < data.size() * 2)
	{
		if (swap)
			mergeByStep(buf, data, subSpanSize);
		else
			mergeByStep(data, buf, subSpanSize);

		subSpanSize *= 2;
		swap = !swap;
	}

	if (swap)
		data = buf;
}

template <typename T>
void mergeSort(std::span<T> data)
{
	std::vector<T> buf(data.size());
	mergeSort(data, std::span<int>{ buf });
}
