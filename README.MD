Day 70: Deadlock
<br>
In computer science, deadlock refers to a specific condition when two or more processes are each waiting for another to release a resource, or more than two processes are waiting for resources in a circular chain (see Necessary conditions). Deadlock is a common problem in multiprocessing where many processes share a specific type of mutually exclusive resource known as a software, or soft, lock. Computers intended for the time-sharing and/or real-time markets are often equipped with a hardware lock (or hard lock) which guarantees exclusive access to processes, forcing serialization. Deadlocks are particularly troubling because there is no general solution to avoid (soft) deadlocks.
<br>
This situation may be understood by an analogy with two people who are drawing diagrams, with only one pencil and one ruler between them. If one person takes the pencil and the other takes the ruler, a deadlock occurs when the person with the pencil needs the ruler and the person with the ruler needs the pencil, before he can give up the ruler. Both requests can't be satisfied, so a deadlock occurs.
<br>
The telecommunications description of deadlock is a little stronger: deadlock occurs when none of the processes meet the condition to move to another state (as described in the process's finite state machine) and all the communication channels are empty. The second condition is often left out on other systems but is important in the telecommunication context and its systems.
<br>
run:
```
fn main() {
    deadlock();
}
```

result:
```
thread:: main
thread:: main
thread:: main
thread:: main
thread:: child
thread:: main
thread:: child
thread:: main
thread:: child

```