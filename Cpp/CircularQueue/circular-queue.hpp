#ifndef CPP_CIRCULARQUEUE_CIRCULAR_QUEUE
#define CPP_CIRCULARQUEUE_CIRCULAR_QUEUE

#include <iostream>
#include <utility>
#include <cstdlib>
#include <cassert>



// A storage that manages slice of memory, but not a data (doesn't construct or destruct objects)
template <typename T>
class MemoryStorage;


template <typename T>
class Queue
{
public:

	Queue()
		: m_begin{ 0 }, m_end{ 0 }
	{}

	Queue(const Queue& another) = delete;
	Queue& operator=(const Queue& another) = delete;

	bool isEmpty() const
	{
		return m_begin == m_end;
	}

	std::size_t length() const
	{
		return firstPartLength() + secondPartLength();
	}

	void reserve(std::size_t capacity)
	{
		MemoryStorage<T> grownStorage{ capacity };
		T* grownStorageWrite{ grownStorage.buffer() };

		// copy first part
		for (std::size_t i{ 0 }; i < firstPartLength(); ++i)
			new (grownStorageWrite++) T{ std::move(buffer()[m_begin + i]) };

		// copy second part
		for (std::size_t i{ 0 }; i < secondPartLength(); ++i)
			new (grownStorageWrite++) T{ std::move(buffer()[i]) };

		m_end = length();
		m_begin = 0;
		m_storage = std::move(grownStorage);
	}

	void pushBack(T value)
	{
		if (isFull())
			grow();

		if (m_end == capacity())
			m_end = 0;

		new (buffer() + m_end) T{ std::move(value) };
		++m_end %= capacity() + 1;
	}

	T popFront()
	{
		if (isEmpty())
			throw std::runtime_error{ "Queue is empty" };

		std::size_t oldBegin{ m_begin };
		++m_begin %= capacity();
		return std::move(buffer()[oldBegin]);
	}

	virtual ~Queue()
	{
		drop();
	}


private:

	T* buffer()
	{
		return m_storage.buffer();
	}

	std::size_t capacity() const
	{
		return m_storage.capacity();
	}

	bool isFull() const
	{
		return length() + 1 >= capacity();
	}

	std::size_t firstPartLength() const
	{
		return (m_end < m_begin ? capacity() : m_end) - m_begin;
	}

	std::size_t secondPartLength() const
	{
		return m_end < m_begin ? m_end : 0;
	}

	void drop()
	{
		if (buffer() == nullptr)
			return;

		// drop fisrt part
		for (std::size_t i{ 0 }; i < firstPartLength(); ++i)
			buffer()[m_begin + i].~T();

		// drop second
		for (std::size_t i{ 0 }; i < secondPartLength(); ++i)
			buffer()[i].~T();
	}

	void grow()
	{
		reserve(capacity() * 2 + 1);
	}

	MemoryStorage<T> m_storage;

	// m_begin points to first element
	// m_end points after last element
	// if m_begin is equal to m_end then queue is empty
	std::size_t m_begin;
	std::size_t m_end;
};


template <typename T>
class MemoryStorage
{
public:

	MemoryStorage()
		: m_capacity{ 0 },
		m_buffer{ nullptr }
	{}

	MemoryStorage(std::size_t capacity)
		: m_capacity{ capacity },
		m_buffer{ static_cast<T*>(std::aligned_alloc(alignof(T), capacity * sizeof(T))) }
	{}

	MemoryStorage(MemoryStorage<T>&& another)
		: m_capacity{ std::exchange(another.m_capacity, 0) },
		m_buffer{ std::exchange(another.m_buffer, nullptr) }
	{}

	MemoryStorage<T>& operator=(MemoryStorage<T>&& another) noexcept
	{
		drop();

		m_capacity = std::exchange(another.m_capacity, 0);
		m_buffer = std::exchange(another.m_buffer, nullptr);

		return *this;
	}

	MemoryStorage(const MemoryStorage& another) = delete;
	MemoryStorage& operator=(const MemoryStorage& another) = delete;

	T* buffer()
	{
		return m_buffer;
	}

	std::size_t capacity() const
	{
		return m_capacity;
	}

	virtual ~MemoryStorage()
	{
		drop();
	}


private:

	void drop()
	{
		free(m_buffer);
		m_capacity = 0;
	}

	T* m_buffer;
	std::size_t m_capacity;
};

#endif /* CPP_CIRCULARQUEUE_CIRCULAR_QUEUE */
