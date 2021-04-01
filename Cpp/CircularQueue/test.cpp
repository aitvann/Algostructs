#include "circular-queue.hpp"

void test()
{
    Queue<int> queue;

	queue.pushBack(1);
	queue.pushBack(2);
	queue.pushBack(3);
	queue.pushBack(4);
	queue.pushBack(5);
	queue.pushBack(6);
	assert(queue.length() == 6);

	assert(queue.popFront() == 1);
	assert(queue.popFront() == 2);
	assert(queue.popFront() == 3);
	assert(queue.popFront() == 4);
	assert(queue.length() == 2);

	queue.pushBack(7);
	queue.pushBack(8);
	queue.pushBack(9);
	queue.pushBack(10); // m_end meets m_begin
	queue.pushBack(11); // resize
	assert(queue.length() == 7);

	assert(queue.popFront() == 5);
	assert(queue.popFront() == 6);
	assert(queue.popFront() == 7);
	assert(queue.popFront() == 8);
	assert(queue.popFront() == 9);
	assert(queue.popFront() == 10);
	assert(queue.popFront() == 11);
	assert(queue.length() == 0);
}
