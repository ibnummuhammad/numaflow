/*
Copyright 2022 The Numaproj Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

package reconciler

import (
	"github.com/prometheus/client_golang/prometheus"
	ctrlmetrics "sigs.k8s.io/controller-runtime/pkg/metrics"

	"github.com/numaproj/numaflow/pkg/metrics"
)

var (
	BuildInfo = prometheus.NewGaugeVec(prometheus.GaugeOpts{
		Subsystem: "controller",
		Name:      "build_info",
		Help:      "A metric with a constant value '1', labeled by controller version and platform from which Numaflow was built",
	}, []string{metrics.LabelVersion, metrics.LabelPlatform})

	ISBSvcHealth = prometheus.NewGaugeVec(prometheus.GaugeOpts{
		Subsystem: "controller",
		Name:      "isbsvc_health",
		Help:      "A metric to indicate whether the ISB Service is healthy. '1' means healthy, '0' means unhealthy",
	}, []string{metrics.LabelNamespace, metrics.LabelISBService})

	PipelineHealth = prometheus.NewGaugeVec(prometheus.GaugeOpts{
		Subsystem: "controller",
		Name:      "pipeline_health",
		Help:      "A metric to indicate whether the Pipeline is healthy. '1' means healthy, '0' means unhealthy",
	}, []string{metrics.LabelNamespace, metrics.LabelISBService})
)

func init() {
	ctrlmetrics.Registry.MustRegister(BuildInfo, ISBSvcHealth, PipelineHealth)
}