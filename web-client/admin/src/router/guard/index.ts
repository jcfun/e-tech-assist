import useFlexMainHeightGuard from './flexMainHeight';
import usePermGuard from './permGuard';

export default function initRouterGuard() {
  usePermGuard();
  useFlexMainHeightGuard();
}
