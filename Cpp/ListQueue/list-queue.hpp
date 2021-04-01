#ifndef LISTQUEUE_LIST_QUEUE
#define LISTQUEUE_LIST_QUEUE

#include <iostream>
#include <cassert>



template <typename T>
struct Node
{
	Node(T value)
		: value{ std::move(value) }, next{ nullptr }
	{}

	T value;
	Node<T>* next;
};


template <typename T>
class Queue
{
public:
	Queue()
		: m_head{ nullptr }, m_tail{ nullptr }, m_length{ 0 }
	{}

	Queue(const Queue& another) = delete;
	Queue& operator=(const Queue& another) = delete;

	bool isEmpty() const
	{
		return length() == 0;
	}

	std::size_t length() const
	{
		return m_length;
	}

	void pushBack(T value)
	{
		Node<T>* node{ new Node<T>{ std::move(value) } };

		if (isEmpty())
			m_head = node;
		else
			m_tail->next = node;

		m_tail = node;
		++m_length;
	}

	T popFront()
	{
		if (isEmpty())
			throw std::runtime_error{ "Queue is empty" };

		T frontCopy{ m_head->value };
		Node<T>* next{ m_head->next };
		delete m_head;

		m_head = next;
		--m_length;
		return std::move(frontCopy);
	}

	virtual ~Queue()
	{
		Node<T>* curr{ m_head };
		m_head = m_tail = nullptr; // just storing invalid pointer is UB
		while(curr != nullptr)
		{
			Node<T>* next{ curr->next };
			delete curr;
			curr = next;
		}

		m_length = 0;
	}


private:

	Node<T>* m_head;
	Node<T>* m_tail;
	std::size_t m_length;
};

#endif /* LISTQUEUE_LIST_QUEUE */
