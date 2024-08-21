package forward

import (
	"math"
	"time"

	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"

	dfv1 "github.com/numaproj/numaflow/pkg/apis/numaflow/v1alpha1"
)

const (
	DefaultRetryFactor        = int32(1)
	DefaultRetryJitter        = int32(0)
	DefaultRetrySteps         = int32(math.MaxInt32)
	DefaultRetrySleepInterval = 1 * time.Millisecond
)

var DefaultRetryDuration = &metav1.Duration{Duration: DefaultRetrySleepInterval}

func defaultRetryStrategy() *dfv1.RetryStrategy {
	factor := DefaultRetryFactor
	jitter := DefaultRetryJitter
	steps := DefaultRetrySteps
	return &dfv1.RetryStrategy{
		BackOff: &dfv1.Backoff{
			Duration: DefaultRetryDuration,
			Factor:   &factor,
			Jitter:   &jitter,
			Steps:    &steps,
			Cap:      DefaultRetryDuration,
		},
		OnFailure: dfv1.OnFailDrop,
	}
}
