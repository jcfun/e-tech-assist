import useCachedGuard from './cache';
import useFlexMainHeightGuard from './flexMainHeight';
import usePermGuard from './permGuard';
import useVisitedGuard from './visited';

export default function initRouterGuard() {
  usePermGuard();
  useVisitedGuard();
  useCachedGuard();
  useFlexMainHeightGuard();
}
