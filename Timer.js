'use strict';

import { v4 } from 'uuid';
import { performance } from 'perf_hooks';

class Timer {
  constructor() {
    /**
     * @type {{_id: string, time: number}[]}
     */
    this.instances = [];
    this.maxTime = 10_000;
  }

  time() {
    const _id = v4();
    this.instances.push({ _id, time: performance.now() });
    return _id;
  }

  /**
   *
   * @param {string} _id
   * @param {boolean} log
   * @param {(timeTaken: string) => unknown} fn
   * @returns {number|string}
   */
  timeEnd(_id, log, fn = (t) => `TimeTaken: ${t}`, onlyVal) {
    const timeObject = this.instances.find((val) => val._id === _id);
    this.instances = this.instances.filter((v) => v._id !== _id);
    if (onlyVal) return performance.now() - (timeObject?.time ?? 0);

    const timeTaken = timeObject
      ? this._formatMs(performance.now() - timeObject.time)
      : 'N/A';
    // eslint-disable-next-line no-console
    if (log) console.log(fn(timeTaken));
    return timeTaken;
  }

  _formatMs(time, digits = 2) {
    if (time >= 1_000) return `${(time / 1_000).toFixed(digits)}s`;
    if (time >= 1) return `${time.toFixed(digits)}ms`;
    return `${(time * 1_000).toFixed(digits)}μs`;
  }
}

// do default export
export { Timer };
