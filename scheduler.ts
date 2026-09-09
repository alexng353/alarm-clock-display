type Task = {
  task: () => void,
  frequency: number // ms
}

const update_display: Task = {
  task: () => { },
  frequency: 1
}

const change_time: Task = {
  task: () => { },
  frequency: 1000
}

type Schedule = {
  time: Date,
  task: Task,
}

const schedules: Schedule[] = [
  {
    time: new Date(0),
    task: update_display
  },
  {
    time: new Date(0),
    task: change_time,
  }
];

function next_task_time() {
  return schedules[0].time;
}

function do_next_task() {
  let now = Date.now();
  if (now >= next_task_time().getTime()) {
    let task = schedules[0];
    task.task.task();
    schedules.push({ task: task.task, time: new Date(Date.now() + task.task.frequency) })
  }
}
